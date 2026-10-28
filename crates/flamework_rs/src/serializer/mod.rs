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
//!   parameter, after [`prepare_serializers`] built every serializer of the file in `begin_file`;
//! - [`union_member_order`]: a union's members in the serializer's order, for `Many<(A | B)[]>` (`crate::macros`);
//! - [`build_decoder`] / [`build_result_decoder`]: the `network-decoder` / `network-result-decoder` intrinsics;
//! - [`encode_list`] / [`encode_result`]: a call site's inline packing (networking, F5);
//! - [`packing_key`]: a list's wire key, for networking's union-of-members check;
//! - [`check_output`]: the end-of-file self-check (`checkSerializerOutput`).
//!
//! Each hands the hoisted statements to `insert_before_root_statement` itself, named in the printer's order.

mod checks;
mod classify;
mod codegen;
mod hoist;
pub mod ir;
mod layout;
pub mod lower;
mod order;
mod pressure;
pub mod typeinfo;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use rbxts_plugin::ts::{Kind as SyntaxKind, Node, P, SourceFile, Symbol, Type};
use rbxts_plugin::{Cx, LuauExpr, LuauStmt, PResult, PluginDiagnostic};

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
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
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
    /// The file's budget of Flamework locals in the main chunk (`LocalBudget`, keyed by file).
    pub(crate) budget_used: u32,
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
            sticky: None,
            true_kind: None,
            ordering_macro: false,
        }
    }

    /// `LocalBudget.claim(file, count)`.
    pub fn claim_locals(&mut self, count: u32) -> bool {
        if self.budget_used + count > LOCAL_BUDGET {
            return false;
        }
        self.budget_used += count;
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

/// The serializer state of one instance's current file.
#[derive(Default)]
pub struct FileSerializer {
    pub generator: Option<Generator>,
    pub names: crate::ts_names::TsNames,
    /// Serializers built ahead with the others of their file ([`prepare_serializers`]), by call: `T` and the value,
    /// in build order.
    pub prepared: HashMap<P<Node>, Vec<(P<Type>, LuauExpr)>>,
    /// What each root statement's serializers hoisted, by the statement's first site, inserted at that site.
    pub pending_hoists: HashMap<P<Node>, Vec<LuauStmt>>,
    /// Whether the file's serializer sites were prepared ([`prepare_serializers`]).
    pub file_prepared: bool,
    /// The prepared root statements' code, built but not yet all named and lowered, in file order, and how many of
    /// them have their hoisted statements done ([`enter_statement`]).
    groups: Vec<Group>,
    groups_done: usize,
    /// The group of each prepared site's call, and of each root statement with sites.
    group_of: HashMap<P<Node>, usize>,
    group_of_statement: HashMap<P<Node>, usize>,
}

/// One root statement's prepared serializers: what its sites hoisted, each site's value (`None` for a site that
/// failed) in build order, and how many of the values are named and lowered.
#[derive(Default)]
struct Group {
    hoisted: Vec<S>,
    objects: Vec<(P<Node>, P<Type>, Option<E>)>,
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
fn run<T>(
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
    let group = fw.serializer.borrow().group_of.get(&node).copied();
    if let Some(group) = group {
        finish_hoists_through(fw, cx, group);
        name_values_through(fw, cx, group, node, ty);
    }
    let prepared = {
        let mut state = fw.serializer.borrow_mut();
        let entries = state.prepared.get_mut(&node);
        entries.and_then(|entries| entries.iter().position(|(t, _)| *t == ty).map(|at| entries.remove(at).1))
    };
    if let Some(prepared) = prepared {
        let hoists = fw.serializer.borrow_mut().pending_hoists.remove(&node);
        if let Some(hoists) = hoists {
            cx.insert_before_root_statement(hoists);
        }
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

/// A `createSerializer` site for [`prepare_serializers`]: the call, `T` and the version.
#[derive(Clone, Copy, Debug)]
pub struct SerializerSite {
    pub node: P<Node>,
    pub ty: P<Type>,
    pub version: Option<u32>,
}

/// Builds every serializer of a file together, in the order the TS transformer met them, and keeps each for its
/// call's [`build_serializer`], which also places what its root statement hoisted. `groups` are the root statements
/// with sites, in file order, each with its sites.
///
/// TypeScript's printer named the generated identifiers in print order, after the declarations of every top-level
/// statement of the file (`generateNames`), so a site's names depend on the sites after it: what a later site
/// hoists to the top of the file (an enum table's `item`, a guard) takes its name first, and within one root
/// statement everything its sites hoisted prints ahead of all their values. Built together, the names come out as
/// TS5's. Call it once per file, before any other hook names anything (`begin_file`). A site that fails reports its
/// error here and later gets `nil` (the build fails on the error). A call with several serializers lists one site
/// for each, in build order.
///
/// The file-level declarations of every group are named here. The rest of what a root statement hoisted is named at
/// the statement's first Flamework hook ([`enter_statement`]), as the printer printed it ahead of the statement, and
/// each site's value when the site is built ([`build_serializer`]), where the printer printed it: what other hooks
/// named before (an earlier statement's code, a guard earlier in the same statement) comes first, as in TS5.
pub fn prepare_serializers(fw: &FwInstance, cx: &mut Cx<'_>, groups: &[(P<Node>, Vec<SerializerSite>)]) {
    let mut built: Vec<Group> = Vec::new();
    for (_, sites) in groups {
        let mut hoisted = Vec::new();
        let mut objects = Vec::new();
        for site in sites {
            let written = written_type_argument(site.node);
            let result = run(fw, cx, site.node, |gn| {
                let value = gn.unwrap_promise(site.ty);
                let versioned = site.version.map(|version| (version, gn.serializer_name(written, value)));
                let shape = gn.spell(written, value)?;
                gn.build_serializer(shape, versioned)
            });
            match result {
                Ok((object, more)) => {
                    hoisted.extend(more);
                    objects.push((site.node, site.ty, Some(object)));
                }
                Err(_) => objects.push((site.node, site.ty, None)),
            }
        }
        built.push(Group { hoisted, objects, named: 0 });
    }

    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    // The printer named the declarations of the file's top-level statements, the hoisted ones included, before it
    // printed anything; the rest of each root statement's code it named as it printed it.
    let mut namer = lower::Namer { ids: &generator.ids, names: &mut generator.names, ts_names: &mut state.names };
    for group in &built {
        namer.generate_top(cx, &group.hoisted);
    }
    for (index, group) in built.iter().enumerate() {
        state.group_of_statement.insert(groups[index].0, index);
        for (node, ..) in &group.objects {
            state.group_of.insert(*node, index);
        }
    }
    state.groups_done = 0;
    state.groups = built;
}

/// At a Flamework hook on `node` (every call and class hook runs it first): names and lowers what the serializers of
/// `node`'s root statement hoisted, and of the root statements before it, unless done. TypeScript's printer printed a
/// root statement's hoisted code ahead of the statement, so it named it before anything a hook of the statement
/// made (a guard's locals, say).
pub fn enter_statement(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>) {
    let mut statement = node;
    while let Some(parent) = statement.parent()
        && parent.kind() != SyntaxKind::SourceFile
    {
        statement = parent;
    }
    let group = fw.serializer.borrow().group_of_statement.get(&statement).copied();
    if let Some(group) = group {
        finish_hoists_through(fw, cx, group);
    }
}

/// Names and lowers the hoisted statements of the prepared root statements up to `last` (in file order). They wait
/// in `pending_hoists` for each statement's first site.
fn finish_hoists_through(fw: &FwInstance, cx: &mut Cx<'_>, last: usize) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    while state.groups_done <= last && state.groups_done < state.groups.len() {
        let group = &mut state.groups[state.groups_done];
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
        if let Some((first, ..)) = group.objects.first() {
            state.pending_hoists.insert(*first, top);
        }
    }
}

/// Names and lowers the values of `group` up to its site (`node`, `ty`), in build order, unless done; they wait in
/// `prepared` for their sites. A user macro's values print in build order, which its hook may not build them in
/// (`wrap<A>(wrap<B>(0))` asks for `A` first): asking for one names those before it too.
fn name_values_through(fw: &FwInstance, cx: &mut Cx<'_>, group: usize, node: P<Node>, ty: P<Type>) {
    let mut state = fw.serializer.borrow_mut();
    let state = &mut *state;
    let Some(generator) = state.generator.as_mut() else { return };
    let group = &mut state.groups[group];
    let Some(at) = group.objects[group.named..].iter().position(|(n, t, _)| *n == node && *t == ty) else { return };
    let through = group.named + at;
    for (node, ty, object) in &group.objects[group.named..=through] {
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
        state.prepared.entry(*node).or_default().push((*ty, value));
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

/// The decoder for an argument list (`network-decoder`): `nil` when the list carries nothing.
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub fn build_decoder(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, ty: P<Type>) -> PResult<LuauExpr> {
    let (decoder, hoisted) = run(fw, cx, node, |gn| {
        let list = gn.list_of(ty, None)?;
        gn.build_decoder(list)
    })?;
    finish_optional(fw, cx, hoisted, decoder)
}

/// The decoder for a function type's result (`network-result-decoder`): `nil` when it carries nothing.
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub fn build_result_decoder(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, function: P<Type>) -> PResult<LuauExpr> {
    let (decoder, hoisted) = run(fw, cx, node, |gn| {
        let list = gn.result_of(function, node)?;
        gn.build_decoder(list)
    })?;
    finish_optional(fw, cx, hoisted, decoder)
}

#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
fn finish_optional(fw: &FwInstance, cx: &mut Cx<'_>, hoisted: Vec<S>, value: Option<E>) -> PResult<LuauExpr> {
    let expressions: Vec<&E> = value.iter().collect();
    let (top, _, mut values) = lower_output(fw, cx, &hoisted, &[], &expressions);
    cx.insert_before_root_statement(top);
    Ok(if values.is_empty() { rbxts_luau::create::nil() } else { values.remove(0) })
}

/// The values a call site packs: one by one, or the table a spread made of them.
#[derive(Debug, Clone)]
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub enum Values {
    /// Each argument's value (an identifier or a literal; read more than once). `parameter` flags the ones that
    /// are parameters of the function around the site.
    List(Vec<(LuauExpr, bool)>),
    Table(LuauExpr),
}

/// A call site's packing: statements to run, then the payload and blob list, when the list carries anything.
#[derive(Debug, Clone)]
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub struct InlineEncoding {
    pub statements: Vec<LuauStmt>,
    pub payload: Option<LuauExpr>,
    pub blobs: Option<LuauExpr>,
}

/// Packs an argument list (`buildInlineEncoding`): `ty` is the member's `_flamework_send` tuple.
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub fn encode_list(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    ty: P<Type>,
    values: Values,
    site: EncodingSite,
) -> PResult<InlineEncoding> {
    let (encoding, hoisted) = run(fw, cx, node, |gn| {
        let list = gn.list_of(ty, None)?;
        let values = gn.external_values(values);
        gn.encode_elements(list, values, site)
    })?;
    finish_encoding(fw, cx, hoisted, encoding)
}

/// Packs a function's result as a one-element list (`buildInlineResultEncoding`).
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub fn encode_result(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    node: P<Node>,
    function: P<Type>,
    value: LuauExpr,
    is_parameter: bool,
    name: Option<String>,
) -> PResult<InlineEncoding> {
    let (encoding, hoisted) = run(fw, cx, node, |gn| {
        let list = gn.result_of(function, node)?;
        let values = gn.external_values(Values::List(vec![(value, is_parameter)]));
        gn.encode_elements(list, values, EncodingSite { name, result: true, outer: 0 })
    })?;
    finish_encoding(fw, cx, hoisted, encoding)
}

#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
fn finish_encoding(fw: &FwInstance, cx: &mut Cx<'_>, hoisted: Vec<S>, encoding: codegen::Encoding) -> PResult<InlineEncoding> {
    let mut expressions = Vec::new();
    let payload = encoding.payload.map(ir::E::Id);
    let blobs = encoding.blobs.map(ir::E::Id);
    if let Some(p) = &payload {
        expressions.push(p);
    }
    if let Some(b) = &blobs {
        expressions.push(b);
    }
    let (top, statements, mut values) = lower_output(fw, cx, &hoisted, &encoding.statements, &expressions);
    cx.insert_before_root_statement(top);
    let payload = payload.map(|_| values.remove(0));
    let blobs = blobs.map(|_| values.remove(0));
    Ok(InlineEncoding { statements, payload, blobs })
}

/// `packingKey`: a list's wire key (a function type's result's with `result`).
#[allow(dead_code, reason = "F5 (networking) calls it: the network intrinsics and call sites")]
pub fn packing_key(fw: &FwInstance, cx: &mut Cx<'_>, node: P<Node>, ty: P<Type>, result: bool) -> PResult<String> {
    let (key, hoisted) = run(fw, cx, node, |gn| {
        let list = if result { gn.result_of(ty, node)? } else { gn.list_of(ty, None)? };
        gn.list_key(list)
    })?;
    let (top, _, _) = lower_output(fw, cx, &hoisted, &[], &[]);
    cx.insert_before_root_statement(top);
    Ok(key)
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
