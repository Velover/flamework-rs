//! The serializer generator (`util/functions/buildSerializerFromType.ts`, 2.0.0-beta.0), ported node for node.
//!
//! The TS generator built TypeScript that roblox-ts compiled; here it builds the same TypeScript as [`ir`] and
//! [`lower`] compiles it the way roblox-ts did, so the Luau is the TS5 pipeline's byte for byte (see `lower` for what
//! the round trip through the printer leaves in the output). The wire format, every order on it, the layout text
//! and its hash are the TS generator's: see the TS file's header, which this port keeps as the reference.
//!
//! **Entry points** (one generator per file and checker, reset by `begin_file`):
//! - [`build_serializer`]: `Flamework.createSerializer<T>()`'s `{ [version,] serialize, deserialize }` (the
//!   `serializer` intrinsic, `crate::macros`), with [`serializer_version`] for the `intrinsic-serializer-options`
//!   parameter, after [`prepare_serializers`] built every site of the file (serializers, network decoders,
//!   networking call sites) in `begin_file`;
//! - [`union_member_order`]: a union's members in the serializer's order, for `Many<(A | B)[]>` (`crate::macros`);
//! - [`build_decoder`]: the `network-decoder` / `network-result-decoder` intrinsics;
//! - [`call_site`]: a networking call site's inline packing ([`crate::networking::calls`] builds it);
//! - [`packing_key`]: a list's wire key, for networking's union-of-members check;
//! - [`check_output`]: the end-of-file self-check (`checkSerializerOutput`).
//!
//! Each hands the hoisted statements to `insert_before_root_statement` itself, named in the printer's order.

mod checks;
mod classify;
pub(crate) mod codegen;
mod hoist;
pub mod ir;
mod layout;
pub mod lower;
mod order;
mod pressure;
pub mod typeinfo;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use rbxts_plugin::ts::{Kind as SyntaxKind, Node, P, SourceFile, Symbol, Type};
use rbxts_plugin::{Cx, LuauExpr, LuauStmt, PResult, PluginDiagnostic};

use crate::networking::calls::{BuiltCall, CallShape};
use crate::plugin::FwInstance;
use crate::setup::FwShared;
use ir::{E, Id, Ids, S};
pub use typeinfo::TableKey;

// --- the generator's model ---------------------------------------------------------------------------------------

/// A number's width on the wire.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Width {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
}

impl Width {
    pub fn size(self) -> u32 {
        match self {
            Width::U8 | Width::I8 => 1,
            Width::U16 | Width::I16 => 2,
            Width::U32 | Width::I32 | Width::F32 => 4,
            Width::U64 | Width::I64 | Width::F64 => 8,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Width::U8 => "u8",
            Width::I8 => "i8",
            Width::U16 => "u16",
            Width::I16 => "i16",
            Width::U32 => "u32",
            Width::I32 => "i32",
            Width::U64 => "u64",
            Width::I64 => "i64",
            Width::F32 => "f32",
            Width::F64 => "f64",
        }
    }

    pub fn from_brand(brand: &str) -> Option<Width> {
        Some(match brand {
            "u8" => Width::U8,
            "i8" => Width::I8,
            "u16" => Width::U16,
            "i16" => Width::I16,
            "u32" => Width::U32,
            "i32" => Width::I32,
            "u64" => Width::U64,
            "i64" => Width::I64,
            "f32" => Width::F32,
            "f64" => Width::F64,
            _ => return None,
        })
    }

    pub fn is_wide(self) -> bool {
        matches!(self, Width::U64 | Width::I64)
    }
}

/// A length prefix: a varint, or a fixed width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LengthWidth {
    V,
    U8,
    U16,
    U32,
}

impl LengthWidth {
    pub fn name(self) -> &'static str {
        match self {
            LengthWidth::V => "v",
            LengthWidth::U8 => "u8",
            LengthWidth::U16 => "u16",
            LengthWidth::U32 => "u32",
        }
    }

    pub fn min(self) -> u32 {
        match self {
            LengthWidth::V => 1,
            LengthWidth::U8 => 1,
            LengthWidth::U16 => 2,
            LengthWidth::U32 => 4,
        }
    }

    pub fn width(self) -> Option<Width> {
        match self {
            LengthWidth::V => None,
            LengthWidth::U8 => Some(Width::U8),
            LengthWidth::U16 => Some(Width::U16),
            LengthWidth::U32 => Some(Width::U32),
        }
    }
}

/// A type, or a kind the generator made up where no type stands (`Shape` in the TS file). Kinds compare by identity,
/// as the TS objects did.
#[derive(Clone, Debug)]
pub enum Shape {
    Type(P<Type>),
    Kind(Rc<Kind>),
}

/// A [`Shape`]'s identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeKey {
    Type(P<Type>),
    Kind(usize),
}

impl Shape {
    pub fn key(&self) -> ShapeKey {
        match self {
            Shape::Type(t) => ShapeKey::Type(*t),
            Shape::Kind(k) => ShapeKey::Kind(Rc::as_ptr(k) as usize),
        }
    }

    pub fn same(&self, other: &Shape) -> bool {
        self.key() == other.key()
    }
}

/// A kind's identity.
pub fn kind_key(kind: &Rc<Kind>) -> usize {
    Rc::as_ptr(kind) as usize
}

/// What the generator knows about a type (`Kind` in the TS file).
#[derive(Debug)]
pub enum Kind {
    Number { width: Width, implicit: bool },
    Varint { implicit: bool },
    Boolean,
    String { length: LengthWidth, implicit: bool },
    Buffer { length: LengthWidth, implicit: bool },
    Constant(E),
    Literals(Vec<E>),
    Nothing,
    Blob { typeof_name: Option<String> },
    Datatype(&'static str),
    CFrame,
    Enum(String),
    Optional(Shape),
    Array(Shape),
    Set(Shape),
    Map(Shape, Shape),
    List { elements: Vec<Shape>, rest: Option<Shape>, after: Option<Vec<Shape>> },
    Object { fields: Vec<Field> },
    Union { alternatives: Vec<Alternative>, ty: Option<P<Type>>, whole: Option<usize> },
}

impl Kind {
    /// The TS `kind.kind` string.
    pub fn name(&self) -> &'static str {
        match self {
            Kind::Number { .. } => "number",
            Kind::Varint { .. } => "varint",
            Kind::Boolean => "boolean",
            Kind::String { .. } => "string",
            Kind::Buffer { .. } => "buffer",
            Kind::Constant(_) => "constant",
            Kind::Literals(_) => "literals",
            Kind::Nothing => "nothing",
            Kind::Blob { .. } => "blob",
            Kind::Datatype(_) => "datatype",
            Kind::CFrame => "cframe",
            Kind::Enum(_) => "enum",
            Kind::Optional(_) => "optional",
            Kind::Array(_) => "array",
            Kind::Set(_) => "set",
            Kind::Map(..) => "map",
            Kind::List { .. } => "list",
            Kind::Object { .. } => "object",
            Kind::Union { .. } => "union",
        }
    }

    pub fn is_table(&self) -> bool {
        matches!(self, Kind::Object { .. } | Kind::Map(..) | Kind::Array(_) | Kind::Set(_) | Kind::List { .. })
    }

    pub fn is_hoistable(&self) -> bool {
        matches!(
            self,
            Kind::Object { .. } | Kind::Union { .. } | Kind::List { .. } | Kind::Array(_) | Kind::Set(_) | Kind::Map(..)
        )
    }

    pub fn tested_ahead(&self) -> bool {
        matches!(
            self,
            Kind::Constant(_) | Kind::Datatype(_) | Kind::CFrame | Kind::Enum(_) | Kind::List { .. } | Kind::Object { .. }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub key: TableKey,
    pub shape: Shape,
}

/// A union member; `ty` is set when the member is a real type.
#[derive(Debug, Clone)]
pub struct Alternative {
    pub shape: Shape,
    pub ty: Option<P<Type>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub size: Option<u64>,
    pub min: u64,
    pub blobs: bool,
    pub zeros: bool,
}

/// `serialization.checks`, defaults filled in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checks {
    pub category: String,
    pub mode: String,
    pub side: String,
    pub types: bool,
}

impl Checks {
    pub fn from_config(shared: &FwShared) -> Checks {
        let checks = shared.loaded.project.get("serialization").and_then(|s| s.get("checks"));
        let text = |key: &str, default: &str| {
            checks.and_then(|c| c.get(key)).and_then(|v| v.as_str()).unwrap_or(default).to_owned()
        };
        let types = checks
            .and_then(|c| c.get("types"))
            .and_then(|v| v.as_bool().or_else(|| v.as_str().map(|s| s == "true")))
            .unwrap_or(false);
        Checks { category: text("category", "implicit"), mode: text("mode", "assert"), side: text("side", "both"), types }
    }
}

/// A hoisted type's functions.
#[derive(Debug, Clone)]
pub struct Hoisted {
    pub name: String,
    pub owner: String,
    pub layout: Layout,
    pub checks: bool,
    pub size_checks: bool,
}

/// Where a guard or table is kept.
#[derive(Debug, Clone)]
pub enum Slot {
    Local(Id),
    Field(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Varint {
    pub size: Id,
    pub write: Id,
    pub read: Id,
}

/// The order a union's members are tested in.
#[derive(Debug, Clone)]
pub struct Evaluation {
    pub order: Vec<usize>,
    pub table_only: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fit {
    None,
    Whole,
    Lossy,
}

/// Where a call site's values go (`EncodingSite`).
#[derive(Debug, Clone, Default)]
pub struct EncodingSite {
    pub name: Option<String>,
    pub result: bool,
    pub outer: u32,
}

/// One file's generator (the TS closure's state). Per checker and per file: `begin_file` starts a new one.
pub struct Generator {
    pub file: P<SourceFile>,
    pub(crate) ids: Ids,
    pub(crate) names: HashMap<Id, String>,
    pub(crate) diagnostic_node: P<Node>,
    pub(crate) kinds: HashMap<P<Type>, Rc<Kind>>,
    /// Every kind made, kept alive so that their identities are never reused.
    pub(crate) keep: Vec<Rc<Kind>>,
    pub(crate) layouts: HashMap<ShapeKey, Layout>,
    pub(crate) visiting: HashSet<ShapeKey>,
    pub(crate) provisional: u32,
    pub(crate) trail: Vec<P<Type>>,
    pub(crate) declarations: Vec<S>,
    pub(crate) tables: Vec<S>,
    pub(crate) definitions: Vec<S>,
    /// The table field each statement of `tables` / `definitions` defines (`definitionOf`).
    pub(crate) table_defines: Vec<Option<String>>,
    pub(crate) definition_defines: Vec<Option<String>>,
    pub(crate) emitted: [usize; 3],
    /// Hoisted types, in insertion order.
    pub(crate) hoisted: Vec<(P<Type>, Rc<Hoisted>)>,
    pub(crate) hoisted_names: HashSet<String>,
    pub(crate) function_table: Option<Id>,
    pub(crate) called: Vec<(String, P<Node>, String)>,
    pub(crate) handed_out: HashSet<String>,
    pub(crate) guards: HashMap<P<Type>, Slot>,
    pub(crate) enum_tables: HashMap<String, Slot>,
    pub(crate) literal_tables: HashMap<usize, (Slot, Slot)>,
    pub(crate) table_fields: HashMap<String, u32>,
    pub(crate) discriminants: HashMap<usize, Option<String>>,
    pub(crate) evaluations: HashMap<usize, Rc<Evaluation>>,
    pub(crate) fits: HashMap<(usize, usize), Fit>,
    pub(crate) warned_unions: HashSet<ShapeKey>,
    pub(crate) warned: HashSet<String>,
    pub(crate) uses: HashMap<ShapeKey, u32>,
    pub(crate) walked: HashSet<ShapeKey>,
    pub(crate) hints: HashMap<P<Type>, String>,
    pub(crate) written_as: HashMap<P<Type>, P<Node>>,
    pub(crate) union_alternatives: HashMap<P<Type>, Rc<(bool, Vec<Alternative>)>>,
    pub(crate) wire_keys: HashMap<ShapeKey, String>,
    pub(crate) layout_keys: HashMap<ShapeKey, String>,
    pub(crate) varint: Option<Varint>,
    pub(crate) zeros: Option<Id>,
    pub(crate) checks: Checks,
    pub(crate) check_function: bool,
    pub(crate) type_check_function: bool,
    pub(crate) checkable_types: Option<Option<P<Type>>>,
    pub(crate) parameters: HashSet<Id>,
    pub(crate) file_level: u32,
    pub(crate) compact: bool,
    pub(crate) hidden_here: HashMap<String, Option<P<Symbol>>>,
    pub(crate) global_aliases: HashMap<String, Id>,
    /// The file's budget of Flamework locals in the main chunk (`LocalBudget`, keyed by file): what the file's
    /// hoisted helpers claimed (live to the file's end), and what its networking call sites claimed ahead of their
    /// statements.
    pub(crate) budget_used: u32,
    pub(crate) budget_placed: u32,
    /// The function scopes' budgets and every claim's statement, for networking's call sites.
    pub(crate) net_budget: crate::networking::scope::Budget,
    /// A refusal recorded while building (a hidden global), returned when the value is done.
    pub(crate) sticky: Option<rbxts_plugin::Abort>,
    /// `TRUE`: a set's values, seen as a map's.
    pub(crate) true_kind: Option<Rc<Kind>>,
    /// Ordering a macro's union ([`union_member_order`]): a failure names the macro, not a serializer.
    pub(crate) ordering_macro: bool,
}

/// `LOCAL_BUDGET`: Flamework locals a file's main chunk may take before they go into the file's table.
pub const LOCAL_BUDGET: u32 = 16;

impl Generator {
    pub fn new(file: P<SourceFile>, checks: Checks) -> Generator {
        Generator {
            file,
            ids: Ids::default(),
            names: HashMap::new(),
            diagnostic_node: file.as_node(),
            kinds: HashMap::new(),
            keep: Vec::new(),
            layouts: HashMap::new(),
            visiting: HashSet::new(),
            provisional: 0,
            trail: Vec::new(),
            declarations: Vec::new(),
            tables: Vec::new(),
            definitions: Vec::new(),
            table_defines: Vec::new(),
            definition_defines: Vec::new(),
            emitted: [0; 3],
            hoisted: Vec::new(),
            hoisted_names: HashSet::new(),
            function_table: None,
            called: Vec::new(),
            handed_out: HashSet::new(),
            guards: HashMap::new(),
            enum_tables: HashMap::new(),
            literal_tables: HashMap::new(),
            table_fields: HashMap::new(),
            discriminants: HashMap::new(),
            evaluations: HashMap::new(),
            fits: HashMap::new(),
            warned_unions: HashSet::new(),
            warned: HashSet::new(),
            uses: HashMap::new(),
            walked: HashSet::new(),
            hints: HashMap::new(),
            written_as: HashMap::new(),
            union_alternatives: HashMap::new(),
            wire_keys: HashMap::new(),
            layout_keys: HashMap::new(),
            varint: None,
            zeros: None,
            checks,
            check_function: false,
            type_check_function: false,
            checkable_types: None,
            parameters: HashSet::new(),
            file_level: 0,
            compact: false,
            hidden_here: HashMap::new(),
            global_aliases: HashMap::new(),
            budget_used: 0,
            budget_placed: 0,
            net_budget: Default::default(),
            sticky: None,
            true_kind: None,
            ordering_macro: false,
        }
    }

    /// `LocalBudget.claim(file, count)`.
    pub fn claim_locals(&mut self, count: u32) -> bool {
        if self.budget_used + self.budget_placed + count > LOCAL_BUDGET {
            return false;
        }
        self.budget_used += count;
        true
    }

    /// `LocalBudget.claim(file, count, statement)`: a networking call site's locals ahead of a top-level statement.
    pub fn claim_file_locals(&mut self, count: u32, statement: P<Node>) -> bool {
        if self.budget_used + self.budget_placed + count > LOCAL_BUDGET {
            return false;
        }
        self.budget_placed += count;
        self.net_budget.place(count, statement);
        true
    }
}

/// The generator at work: its state, the hook's context and the build's shared state.
pub(crate) struct Gen<'g, 'c, 'x> {
    pub g: &'g mut Generator,
    pub cx: &'c mut Cx<'x>,
    pub shared: &'g FwShared,
}

/// A failed value: the error is recorded.
pub(crate) type GResult<T> = PResult<T>;

// --- the per-instance state ---------------------------------------------------------------------------------------

/// Which intrinsic a prepared value is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueTag {
    Serializer,
    Decoder,
    ResultDecoder,
}

/// The serializer state of one instance's current file.
#[derive(Default)]
pub struct FileSerializer {
    pub generator: Option<Generator>,
    pub names: crate::ts_names::TsNames,
    /// Values built ahead with the others of their file ([`prepare_serializers`]), by call: `T`, the intrinsic and the
    /// value, in build order.
    pub prepared: HashMap<P<Node>, Vec<PreparedValue>>,
    /// What each root statement's sites hoisted, by group, inserted at the group's first site that is lowered.
    pending_hoists: HashMap<usize, Vec<LuauStmt>>,
    /// Whether the file's sites were prepared ([`prepare_serializers`]).
    pub file_prepared: bool,
    /// The prepared root statements' code, built but not yet all named and lowered, in file order, and how many of
    /// them have their hoisted statements done ([`enter_statement`]).
    groups: Vec<Group>,
    groups_done: usize,
    /// The group of each prepared site's call, and of each root statement with sites.
    group_of: HashMap<P<Node>, usize>,
    group_of_statement: HashMap<P<Node>, usize>,
    /// The networking call sites built ahead, by call.
    calls: HashMap<P<Node>, Rc<RefCell<BuiltCall>>>,
    /// The call sites whose packing lands ahead of a statement, by the function whose printer pass names their
    /// locals (the body's `generateNames`), in file order.
    scope_calls: HashMap<P<Node>, Vec<P<Node>>>,
    /// The functions (and the file) whose call sites' locals are named.
    entered: HashSet<P<Node>>,
    /// The class members whose call sites' locals are named ([`enter_members_before`]).
    members_entered: HashSet<P<Node>>,
}

/// One root statement's prepared sites: what they hoisted, each value's (`None` for a site that failed, or a decoder
/// of a list that carries nothing) in build order, and how many of the values are named and lowered.
/// A value built ahead: `T`, the intrinsic and the value.
pub type PreparedValue = (P<Type>, ValueTag, LuauExpr);
/// A group's site of a value: the call, `T`, the intrinsic and the value once built.
type GroupObject = (P<Node>, P<Type>, ValueTag, Option<E>);
/// A packed call site built ahead, by its call.
type PreparedCall = (P<Node>, Rc<RefCell<BuiltCall>>);

#[derive(Default)]
struct Group {
    hoisted: Vec<S>,
    objects: Vec<GroupObject>,
    named: usize,
}

// --- entry points -------------------------------------------------------------------------------------------------

/// Names and lowers a hook's output: `top` goes before the current root statement, `inline` and `expressions`
/// where the hook's value is.
fn lower_output(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    top: &[S],
    inline: &[S],
    expressions: &[&E],
) -> (Vec<LuauStmt>, Vec<LuauStmt>, Vec<LuauExpr>) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let generator = state.generator.as_mut().expect("a generator");
    {
        let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
        namer.name_output(cx, top, inline, expressions);
    }
    let mut temp = |hint: &str| cx.temp_id(hint);
    let mut lowerer = lower::Lowerer { ids: &generator.ids, names: &generator.names, temp: &mut temp };
    let top = lowerer.statements(top);
    let inline_stmts = lowerer.statements(inline);
    let mut prereqs = Vec::new();
    let values = expressions.iter().map(|e| lowerer.expr(e, &mut prereqs)).collect();
    let mut inline_all = inline_stmts;
    inline_all.extend(prereqs);
    (top, inline_all, values)
}

/// Runs a build of the generator and returns the hoisted statements with its result.
pub(crate) fn run<T>(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    f: impl FnOnce(&mut Gen<'_, '_, '_>) -> GResult<T>,
) -> GResult<(T, Vec<S>)> {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    if state.generator.is_none() {
        let file = cx.file().source;
        state.generator = Some(Generator::new(file, Checks::from_config(&fw.shared)));
        state.names = crate::ts_names::TsNames::with_texts(cx.file_identifier_texts());
    }
    let generator = state.generator.as_mut().unwrap();
    let mut gn = Gen { g: generator, cx, shared: &fw.shared };
    gn.use_node(node);
    let mut result = f(&mut gn);
    if let Some(abort) = gn.g.sticky.take() {
        result = Err(abort);
    }
    // What was hoisted is handed out even when the value failed: the TS transformer queued it before the error
    // reached the statement, and a later value of the file may call it.
    let hoisted = gn.take_hoisted();
    result.map(|value| (value, hoisted))
}

/// `Flamework.createSerializer<T>()`: `{ serialize, deserialize }`, with `version` (and a header in every buffer)
/// for a versioned serializer. `node` is the macro call (its first type argument, as written, numbers the unions in
/// it); `ty` the intrinsic's input `T`; `version` from [`serializer_version`].
pub fn build_serializer(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    ty: P<Type>,
    version: Option<u32>,
) -> PResult<LuauExpr> {
    if let Some(prepared) = take_prepared(fw, cx, node, ty, ValueTag::Serializer) {
        return Ok(prepared);
    }
    let written = written_type_argument(node);
    let (object, hoisted) = run(fw, cx, node, |gn| {
        let value = gn.unwrap_promise(ty);
        let versioned = version.map(|version| (version, gn.serializer_name(written, value)));
        let shape = gn.spell(written, value)?;
        gn.build_serializer(shape, versioned)
    })?;
    let (top, _, mut values) = lower_output(fw, cx, &hoisted, &[], &[&object]);
    cx.insert_before_root_statement(top);
    Ok(values.remove(0))
}

/// The value prepared for the site (`node`, `ty`, `tag`), named and lowered with the values before it, with what its
/// root statement hoisted placed; `None` for a site that was not prepared.
fn take_prepared(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, ty: P<Type>, tag: ValueTag) -> Option<LuauExpr> {
    let group = fw.serializer.borrow().group_of.get(&node).copied();
    if let Some(group) = group {
        finish_hoists_through(fw, cx, group);
        name_values_through(fw, cx, group, node, ty, tag);
    }
    let prepared = {
        let mut state = fw.serializer.borrow_mut();
        let entries = state.prepared.get_mut(&node);
        entries.and_then(|entries| {
            entries.iter().position(|(t, g, _)| *t == ty && *g == tag).map(|at| entries.remove(at).2)
        })
    }?;
    if let Some(group) = group {
        flush_hoists(fw, cx, group);
    }
    Some(prepared)
}

/// Places what a group's sites hoisted, at the first of them that is lowered.
fn flush_hoists(fw: &FwInstance, cx: &mut Cx<'_>, group: usize) {
    let hoists = fw.serializer.borrow_mut().pending_hoists.remove(&group);
    if let Some(hoists) = hoists {
        cx.insert_before_root_statement(hoists);
    }
}

/// A site for [`prepare_serializers`]: the call (or the macro call an intrinsic is in), `T` and what it builds.
#[derive(Clone, Debug)]
pub struct SerializerSite {
    pub node: P<Node>,
    pub ty: P<Type>,
    pub kind: SiteKind,
}

/// What a prepared site builds.
#[derive(Clone, Debug)]
pub enum SiteKind {
    /// `createSerializer<T>()`, with its version.
    Serializer { version: Option<u32> },
    /// `network-decoder` (`T` the argument list) or, with `result`, `network-result-decoder` (`T` the function type).
    Decoder { result: bool },
    /// A networking call site's packing (`T` unused: the call's shape says what is packed).
    Call(Rc<CallShape>),
}

impl SiteKind {
    /// The kind with the call's serializer version (a `createSerializer` site's).
    pub fn with_version(&self, version: Option<u32>) -> SiteKind {
        match self {
            SiteKind::Serializer { .. } => SiteKind::Serializer { version },
            other => other.clone(),
        }
    }
}

/// Builds every site of a file together, in the order the TS transformer met them, and keeps each for its hook,
/// which also places what its root statement hoisted. `groups` are the root statements with sites, in file order,
/// each with its sites: `createSerializer` values, network decoders and networking call sites, which share the file's
/// hoisted helpers.
///
/// TypeScript's printer named the generated identifiers in print order, after the declarations of every top-level
/// statement of the file (`generateNames`), so a site's names depend on the sites after it: what a later site
/// hoists to the top of the file (an enum table's `item`, a guard) takes its name first, and within one root
/// statement everything its sites hoisted prints ahead of all their values. Built together, the names come out as
/// TS5's. Call it once per file, before any other hook names anything (`begin_file`). A site that fails reports its
/// error here and later gets `nil` (the build fails on the error). A call with several values lists one site for
/// each, in build order.
///
/// The file-level declarations of every group are named here, with the locals the file's own call sites put ahead
/// of their statements (`generateNames` of the file walks into its statements' blocks). The rest of what a root
/// statement hoisted is named at the statement's first Flamework hook ([`enter_statement`]), as the printer printed
/// it ahead of the statement; a function's call sites' locals when its body is entered (the printer's
/// `generateNames` of the body); and each value when its site is built ([`build_serializer`]), where the printer
/// printed it: what other hooks named before (an earlier statement's code, a guard earlier in the same statement)
/// comes first, as in TS5.
pub fn prepare_serializers(fw: &FwInstance, cx: &mut Cx<'_>, groups: &[(P<Node>, Vec<SerializerSite>)]) {
    let mut built: Vec<Group> = Vec::new();
    let mut calls: Vec<Vec<PreparedCall>> = Vec::new();
    for (_, sites) in groups {
        let mut hoisted = Vec::new();
        let mut objects = Vec::new();
        let mut group_calls = Vec::new();
        for site in sites {
            match &site.kind {
                SiteKind::Serializer { version } => {
                    let written = written_type_argument(site.node);
                    let result = run(fw, cx, site.node, |gn| {
                        let value = gn.unwrap_promise(site.ty);
                        let versioned = version.map(|version| (version, gn.serializer_name(written, value)));
                        let shape = gn.spell(written, value)?;
                        gn.build_serializer(shape, versioned)
                    });
                    match result {
                        Ok((object, more)) => {
                            hoisted.extend(more);
                            objects.push((site.node, site.ty, ValueTag::Serializer, Some(object)));
                        }
                        Err(_) => objects.push((site.node, site.ty, ValueTag::Serializer, None)),
                    }
                }
                SiteKind::Decoder { result } => {
                    let tag = if *result { ValueTag::ResultDecoder } else { ValueTag::Decoder };
                    match run(fw, cx, site.node, |gn| decoder(gn, site.node, site.ty, *result)) {
                        Ok((decoder, more)) => {
                            hoisted.extend(more);
                            objects.push((site.node, site.ty, tag, decoder));
                        }
                        Err(_) => objects.push((site.node, site.ty, tag, None)),
                    }
                }
                SiteKind::Call(shape) => {
                    if let Ok((call, more)) =
                        run(fw, cx, site.node, |gn| crate::networking::calls::build(gn, site.node, shape))
                    {
                        hoisted.extend(more);
                        group_calls.push((site.node, Rc::new(RefCell::new(call))));
                    }
                }
            }
        }
        built.push(Group { hoisted, objects, named: 0 });
        calls.push(group_calls);
    }

    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    let file = cx.file().source.as_node();
    // The printer named the declarations of the file's top-level statements, the hoisted ones and the locals the
    // file's own call sites put ahead of their statements included, before it printed anything; the rest of each
    // root statement's code it named as it printed it.
    let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
    for (group, group_calls) in built.iter().zip(&calls) {
        namer.generate_top(cx, &group.hoisted);
        for (_, call) in group_calls {
            let call = call.borrow();
            if call.placement.ahead_of_statement() && call.naming_scope == file {
                call.generate_declarations(&mut namer, cx);
            }
        }
    }
    state.entered.insert(file);
    for (index, (group, group_calls)) in built.iter().zip(calls).enumerate() {
        state.group_of_statement.insert(groups[index].0, index);
        for (node, ..) in &group.objects {
            state.group_of.insert(*node, index);
        }
        for (node, call) in group_calls {
            state.group_of.insert(node, index);
            let scope = {
                let call = call.borrow();
                call.placement.ahead_of_statement().then_some(call.naming_scope)
            };
            if let Some(scope) = scope
                && scope != file
            {
                state.scope_calls.entry(scope).or_default().push(node);
            }
            state.calls.insert(node, call);
        }
    }
    state.groups_done = 0;
    state.groups = built;
}

/// At a Flamework hook on `node` (every call and class hook runs it first): names and lowers what the sites of
/// `node`'s root statement hoisted, and of the root statements before it, unless done, and names the locals the call
/// sites of each function around `node` put ahead of their statements, unless done. TypeScript's printer printed a
/// root statement's hoisted code ahead of the statement, so it named it before anything a hook of the statement
/// made (a guard's locals, say), and it named a function body's declarations when it began to print the body. In a
/// class member, the call sites of the members before it are named first ([`enter_members_before`]).
pub fn enter_statement(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>) {
    let mut statement = node;
    let mut scopes = Vec::new();
    while let Some(parent) = statement.parent()
        && parent.kind() != SyntaxKind::SourceFile
    {
        if matches!(parent.kind(), SyntaxKind::ClassDeclaration | SyntaxKind::ClassExpression)
            && parent.members().contains(&statement)
        {
            scopes.push((parent, Some(statement)));
        }
        if tsrs_ast::is_function_like(Some(parent))
            || parent.kind() == SyntaxKind::ClassStaticBlockDeclaration
            || parent.kind() == SyntaxKind::CallExpression
        {
            scopes.push((parent, None));
        }
        statement = parent;
    }
    let group = fw.serializer.borrow().group_of_statement.get(&statement).copied();
    if let Some(group) = group {
        finish_hoists_through(fw, cx, group);
    }
    // Outermost first, as the printer enters them: a function's body, or the function a call site's packing is
    // wrapped in, whose locals print ahead of anything in the call (its target and arguments).
    for &(scope, member) in scopes.iter().rev() {
        if let Some(member) = member {
            enter_members_before(fw, cx, scope, member);
        } else if scope.kind() == SyntaxKind::CallExpression {
            enter_wrapped_call(fw, cx, scope);
        } else {
            enter_function(fw, cx, scope);
        }
    }
}

/// Names the call sites' locals of the members of `class` before `member`, once each. TypeScript's printer printed a
/// class's members in source order, so it named theirs before it reached `member`; roblox-ts compiles the instance
/// properties' initializers first (in the constructor), then the methods, then the static properties, so their hooks
/// may come later. Within a member, as the printer reached them: a function's sites that land ahead of a statement
/// when it began the function, a wrapped site when it began the site's function.
fn enter_members_before(fw: &FwInstance, cx: &mut Cx<'_>, class: P<Node>, member: P<Node>) {
    let span = |node: P<Node>| (node.pos().source_offset().unwrap_or(0), node.end().source_offset().unwrap_or(0));
    for &earlier in class.members() {
        if earlier == member {
            break;
        }
        let mut entries = Vec::new();
        {
            let mut state = fw.serializer.borrow_mut();
            if !state.members_entered.insert(earlier) {
                continue;
            }
            let (start, end) = span(earlier);
            for (&node, call) in &state.calls {
                let (pos, call_end) = span(node);
                if pos < start || call_end > end {
                    continue;
                }
                let call = call.borrow();
                let (at, wrapped) =
                    if call.placement.ahead_of_statement() { (call.naming_scope, false) } else { (node, true) };
                let (at_pos, at_end) = span(at);
                // Outer before inner where they start together (`f().x.fire(…)` with `f()` a send).
                entries.push((at_pos, std::cmp::Reverse(at_end), wrapped, at));
            }
        }
        entries.sort_by_key(|&(pos, end, wrapped, _)| (pos, end, wrapped));
        entries.dedup_by_key(|entry| entry.3);
        for (_, _, wrapped, at) in entries {
            if wrapped {
                enter_wrapped_call(fw, cx, at);
            } else {
                enter_function(fw, cx, at);
            }
        }
    }
}

/// Names the locals of a call site whose packing is wrapped in a function of its own (or in an optional chain's nil
/// check), once: the printer named them when it began the function, ahead of the call's target and arguments, which
/// the compiler may compile before the call's own hook (the target of `f().x.fire(…)`).
fn enter_wrapped_call(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(call) = state.calls.get(&node) else { return };
    let call = call.borrow();
    if call.placement.ahead_of_statement() {
        return;
    }
    let Some(generator) = state.generator.as_mut() else { return };
    let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
    call.generate_declarations(&mut namer, cx);
}

/// Names the locals of the call sites whose packing lands ahead of a statement of `function`'s body (and of the
/// blocks in it), once.
fn enter_function(fw: &FwInstance, cx: &mut Cx<'_>, function: P<Node>) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    if !state.entered.insert(function) {
        return;
    }
    let Some(nodes) = state.scope_calls.get(&function) else { return };
    let Some(generator) = state.generator.as_mut() else { return };
    let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
    for node in nodes {
        if let Some(call) = state.calls.get(node) {
            call.borrow().generate_declarations(&mut namer, cx);
        }
    }
}

/// Names and lowers the hoisted statements of the prepared root statements up to `last` (in file order). They wait
/// in `pending_hoists` for each group's first site that is lowered.
fn finish_hoists_through(fw: &FwInstance, cx: &mut Cx<'_>, last: usize) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    while state.groups_done <= last && state.groups_done < state.groups.len() {
        let index = state.groups_done;
        let group = &mut state.groups[index];
        state.groups_done += 1;
        let hoisted = std::mem::take(&mut group.hoisted);
        {
            let mut namer =
                lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
            namer.walk_output(cx, &hoisted, &[]);
        }
        let mut temp = |hint: &str| cx.temp_id(hint);
        let mut lowerer = lower::Lowerer { ids: &generator.ids, names: &generator.names, temp: &mut temp };
        let top = lowerer.statements(&hoisted);
        if !top.is_empty() {
            state.pending_hoists.insert(index, top);
        }
    }
}

/// Names and lowers the values of `group` up to its site (`node`, `ty`, `tag`), in build order, unless done; they wait
/// in `prepared` for their sites. A user macro's values print in build order, which its hook may not build them in
/// (`wrap<A>(wrap<B>(0))` asks for `A` first): asking for one names those before it too.
fn name_values_through(fw: &FwInstance, cx: &mut Cx<'_>, group: usize, node: P<Node>, ty: P<Type>, tag: ValueTag) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    let group = &mut state.groups[group];
    let Some(at) =
        group.objects[group.named..].iter().position(|(n, t, g, _)| *n == node && *t == ty && *g == tag)
    else {
        return;
    };
    let through = group.named + at;
    for (node, ty, tag, object) in &group.objects[group.named..=through] {
        let value = match object {
            Some(object) => {
                {
                    let mut namer =
                        lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
                    namer.walk_output(cx, &[], &[object]);
                }
                let mut temp = |hint: &str| cx.temp_id(hint);
                let mut lowerer = lower::Lowerer { ids: &generator.ids, names: &generator.names, temp: &mut temp };
                let mut prereqs = Vec::new();
                lowerer.expr(object, &mut prereqs)
            }
            None => rbxts_luau::create::nil(),
        };
        state.prepared.entry(*node).or_default().push((*ty, *tag, value));
    }
    group.named = through + 1;
}

/// `union`'s members in the serializer's order (`orderAlternatives`): the written order of `written` (the union as the
/// call writes it) or of the alias it was declared as, else by a key of each member's type. `Modding.Many<(A | B)[]>`
/// lists its values in this order. TS5 took its checker's creation order, which is the written one for a union of
/// types the call makes (`({ z: 1 } | { a: 1 })[]`); for members that existed before (`(3 | 1 | 2)[]`, property names
/// of the libraries, `keyof`) its order depends on what its checker met first, and this one is the written or key
/// order instead (DESIGN §10.4). A tie fails the build, as it does for a serializer. `node` is the macro call.
pub fn union_member_order(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    written: Option<P<Node>>,
    union: P<Type>,
) -> PResult<Vec<P<Type>>> {
    let members: Vec<P<Type>> = union.types().to_vec();
    let (ordered, hoisted) = run(fw, cx, node, |gn| {
        let alternatives: Vec<Alternative> =
            members.iter().map(|&t| Alternative { shape: Shape::Type(t), ty: Some(t) }).collect();
        gn.g.ordering_macro = true;
        let ordered = gn.order_alternatives(&alternatives, written.or_else(|| classify::alias_node(union)), union);
        gn.g.ordering_macro = false;
        ordered
    })?;
    debug_assert!(hoisted.is_empty(), "ordering a union hoists nothing");
    Ok(ordered.into_iter().filter_map(|a| a.ty).collect())
}

/// The written first type argument of a call or `new`.
fn written_type_argument(node: P<Node>) -> Option<P<Node>> {
    if node.kind() != SyntaxKind::CallExpression {
        return None;
    }
    node.type_arguments().first().copied()
}

/// `buildDecoderFromType` / `buildResultDecoderFromType`: the decoder for an argument list (`ty` its tuple), or with
/// `result` for a function type's result; `None` when it carries nothing.
fn decoder(gn: &mut Gen<'_, '_, '_>, node: P<Node>, ty: P<Type>, result: bool) -> GResult<Option<E>> {
    let list = if result { gn.result_of(ty, node)? } else { gn.list_of(ty, None)? };
    gn.build_decoder(list)
}

/// The `network-decoder` intrinsic (`network-result-decoder` with `result`): the decoder, or `nil` when the list
/// carries nothing.
pub fn build_decoder(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, ty: P<Type>, result: bool) -> PResult<LuauExpr> {
    let tag = if result { ValueTag::ResultDecoder } else { ValueTag::Decoder };
    if let Some(prepared) = take_prepared(fw, cx, node, ty, tag) {
        return Ok(prepared);
    }
    let (decoder, hoisted) = run(fw, cx, node, |gn| decoder(gn, node, ty, result))?;
    let expressions: Vec<&E> = decoder.iter().collect();
    let (top, _, mut values) = lower_output(fw, cx, &hoisted, &[], &expressions);
    cx.insert_before_root_statement(top);
    Ok(if values.is_empty() { rbxts_luau::create::nil() } else { values.remove(0) })
}

/// A networking call site's code ([`crate::networking::calls`]): the one built when the file began (with what its
/// root statement hoisted placed), or one built now, whose hoisted statements are placed now. The flag says which.
pub(crate) fn call_site(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    shape: &CallShape,
) -> PResult<(Rc<RefCell<BuiltCall>>, bool)> {
    let (prepared, group) = {
        let state = fw.serializer.borrow();
        (state.calls.get(&node).cloned(), state.group_of.get(&node).copied())
    };
    if let Some(prepared) = prepared {
        if let Some(group) = group {
            finish_hoists_through(fw, cx, group);
            flush_hoists(fw, cx, group);
        }
        return Ok((prepared, true));
    }
    let (call, hoisted) = run(fw, cx, node, |gn| crate::networking::calls::build(gn, node, shape))?;
    let (top, ..) = lower_output(fw, cx, &hoisted, &[], &[]);
    cx.insert_before_root_statement(top);
    Ok((Rc::new(RefCell::new(call)), false))
}

/// Runs `f` with the file's namer.
pub(crate) fn with_namer<T>(fw: &FwInstance, f: impl FnOnce(&mut lower::Namer<'_>) -> T) -> T {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let generator = state.generator.as_mut().expect("a generator");
    let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
    f(&mut namer)
}

/// Runs `f` with the file's lowerer (what it lowers is named).
pub(crate) fn with_lowerer<T>(fw: &FwInstance, cx: &mut Cx<'_>, f: impl FnOnce(&mut lower::Lowerer<'_>) -> T) -> T {
    let state = fw.serializer.borrow();
    let generator = state.generator.as_ref().expect("a generator");
    let mut temp = |hint: &str| cx.temp_id(hint);
    let mut lowerer = lower::Lowerer { ids: &generator.ids, names: &generator.names, temp: &mut temp };
    f(&mut lowerer)
}

/// `packingKey`: a list's wire key (a function type's result's with `result`).
pub fn packing_key(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, ty: P<Type>, result: bool) -> PResult<String> {
    let (key, hoisted) = run(fw, cx, node, |gn| gn.packing_key(node, ty, result))?;
    let (top, _, _) = lower_output(fw, cx, &hoisted, &[], &[]);
    cx.insert_before_root_statement(top);
    Ok(key)
}

impl Gen<'_, '_, '_> {
    /// `packingKey(type, result)`.
    pub(crate) fn packing_key(&mut self, node: P<Node>, ty: P<Type>, result: bool) -> GResult<String> {
        let list = if result { self.result_of(ty, node)? } else { self.list_of(ty, None)? };
        self.list_key(list)
    }
}

/// `checkSerializerOutput`, at the end of a file: every field of the file's table the code called has its
/// definition. Call from `end_file` (it reports at the node that first called the field).
pub fn check_output(fw: &FwInstance, cx: &mut Cx<'_>) {
    let mut state = fw.serializer.borrow_mut();
    let Some(generator) = state.generator.as_mut() else { return };
    let missing: Vec<(String, P<Node>, String)> =
        generator.called.iter().filter(|(field, ..)| !generator.handed_out.contains(field)).cloned().collect();
    generator.called.clear();
    generator.handed_out.clear();
    drop(state);
    let mut by_owner: Vec<(String, P<Node>, Vec<String>)> = Vec::new();
    for (field, node, owner) in missing {
        match by_owner.iter_mut().find(|(o, ..)| *o == owner) {
            Some(entry) => entry.2.push(format!("'codec.{field}'")),
            None => by_owner.push((owner, node, vec![format!("'codec.{field}'")])),
        }
    }
    for (owner, node, fields) in by_owner {
        let list = if fields.len() > 1 {
            format!("{} and {}", fields[..fields.len() - 1].join(", "), fields[fields.len() - 1])
        } else {
            fields[0].clone()
        };
        cx.report(
            PluginDiagnostic::error(node, format!("Flamework's generated code for {owner} calls {list}, which this file never defines."))
                .with_hint("This is a bug in Flamework: please report it, with the file. Building again from scratch (delete the output folder) may get past it."),
        );
    }
}

/// `getSerializerVersion`: the version a serializer macro is called with, read off the argument of the parameter
/// the declaration names with `intrinsic-serializer-options` (`options_parameter`). `None` without one.
pub fn serializer_version(cx: &mut Cx<'_>, call: P<Node>, options_parameter: Option<P<Symbol>>) -> PResult<Option<u32>> {
    hoist::serializer_version(cx, call, options_parameter)
}

impl Gen<'_, '_, '_> {
    /// `use(node)`: points diagnostics at the site being built.
    pub fn use_node(&mut self, node: P<Node>) {
        self.g.diagnostic_node = node;
        self.g.hidden_here.clear();
    }
}

/// Parity tooling only (the `layout-dump` cargo feature, off in every normal build): with `FLAMEWORK_RS_LAYOUT_DUMP`
/// set to a file, every versioned serializer appends its type, layout text and hash there (name, hash hex in buffer
/// order and text, tab-separated), for comparing with TS5's.
#[cfg(feature = "layout-dump")]
pub(crate) fn dump_layout(name: &str, text: &str, hash: u32) {
    let Some(path) = std::env::var_os("FLAMEWORK_RS_LAYOUT_DUMP") else { return };
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let bytes = hash.to_le_bytes();
        // One write per line: several checkers append to the same file at once.
        let line = format!("{name}	{}	{}\n", crate::sha::hex(&bytes), text.replace(char::is_whitespace, " "));
        let _ = file.write_all(line.as_bytes());
    }
}
