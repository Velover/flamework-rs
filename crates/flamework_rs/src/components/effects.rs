//! roblox-ts's evaluation-order analysis (`util/evaluation/effects.ts`, as rbxts-rs ports it in
//! `rbxts_transform::util::evaluation::{effects, node}`), over the Luau the compiler hands back: what running it
//! may do, and whether two evaluations can be reordered. The facts it reads (binding keys, with whether a closure can
//! reassign the binding; constant references, stable lookups, call effects) are the ones the compiler recorded on the
//! nodes, so the setter call ([`super::setter`]) holds in locals exactly what roblox-ts holds for the TS's
//! `x[SETTER](k, v)`. Plugins cannot ask the compiler for this yet (API gap 3); this copy goes when they can.

use std::collections::HashSet;

use rbxts_luau::{
    AnyIdentifier, Binding, Effects, ElseBody, Expr, IfStatement, InterpolatedStringPart, MapField, Stmt, TableField,
    UnaryOperator,
};

/// `effectsCommute`, including errors: a failed operand must not allow a later mutation to happen.
pub(crate) fn effects_commute(a: &Effects, b: &Effects) -> bool {
    !(intersects(&a.reads, &b.writes)
        || intersects(&a.writes, &b.reads)
        || intersects(&a.writes, &b.writes)
        || (a.reads_heap && b.writes_heap)
        || (a.writes_heap && (b.reads_heap || b.writes_heap))
        || (a.throws && (b.throws || has_writes(b)))
        || (b.throws && has_writes(a)))
}

/// `joinEffects`
pub(crate) fn join_effects(mut a: Effects, b: &Effects) -> Effects {
    a.reads.extend(b.reads.iter().cloned());
    a.writes.extend(b.writes.iter().cloned());
    a.reads_heap |= b.reads_heap;
    a.writes_heap |= b.writes_heap;
    a.throws |= b.throws;
    a.allocates |= b.allocates;
    a
}

/// `getEffects` of one expression.
pub(crate) fn get_effects(node: &Expr) -> Effects {
    if is_constant_reference(node) || node.is_simple_primitive() || node.is_none() {
        return Effects::none();
    }
    if matches!(node, Expr::Function(_)) {
        return Effects::allocation();
    }
    let mut result = Effects::none();
    accumulate(Node::Expr(node), &mut result);
    result
}

/// `getEffects` of a statement list.
pub(crate) fn get_effects_of_statements(statements: &[Stmt]) -> Effects {
    let mut result = Effects::none();
    for s in statements {
        accumulate(Node::Stmt(s), &mut result);
    }
    result
}

/// `isLateRead`: Luau can use a local's register directly until the consuming instruction.
pub(crate) fn is_late_read(expression: &Expr) -> bool {
    let mut e = expression;
    while let Expr::Parenthesized(p) = e {
        e = &p.expression;
    }
    e.is_any_identifier()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Wildcard {
    Reads,
    Writes,
}

fn is_exposed_binding(key: &Binding, wildcard: Wildcard) -> bool {
    match key {
        Binding::Key(k) => match wildcard {
            Wildcard::Writes => k.written_by_closure,
            Wildcard::Reads => k.captured,
        },
        // compiler temporaries are inaccessible to user code; untracked names fail closed
        Binding::Temp(_) => false,
        Binding::Name(_) | Binding::UserReads | Binding::UserWrites => true,
    }
}

fn intersects(a: &HashSet<Binding>, b: &HashSet<Binding>) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    for (symbol, wildcard) in [(Binding::UserReads, Wildcard::Reads), (Binding::UserWrites, Wildcard::Writes)] {
        if a.contains(&symbol) && b.iter().any(|k| is_exposed_binding(k, wildcard)) {
            return true;
        }
        if b.contains(&symbol) && a.iter().any(|k| is_exposed_binding(k, wildcard)) {
            return true;
        }
    }
    a.iter().any(|k| b.contains(k))
}

fn has_writes(effects: &Effects) -> bool {
    effects.writes_heap || effects.writes.iter().any(|k| !matches!(k, Binding::Temp(_)))
}

fn is_constant_reference(expression: &Expr) -> bool {
    expression.facts().is_some_and(|f| f.is_constant())
}

fn is_primitive_value(node: &Expr) -> bool {
    if node.facts().is_some_and(|f| f.is_primitive()) || node.is_simple_primitive() {
        return true;
    }
    if let Expr::Parenthesized(p) = node {
        return is_primitive_value(&p.expression);
    }
    false
}

fn binding_of_id(identifier: &AnyIdentifier) -> Binding {
    if let Some(key) = identifier.facts().binding() {
        return Binding::Key(key);
    }
    match identifier {
        AnyIdentifier::Identifier(id) => Binding::Name(id.name.clone()),
        AnyIdentifier::Temporary(id) => Binding::Temp(id.id),
    }
}

fn binding_of_expr(expression: &Expr) -> Option<Binding> {
    match expression {
        Expr::Identifier(id) => Some(match id.facts.binding() {
            Some(key) => Binding::Key(key),
            None => Binding::Name(id.name.clone()),
        }),
        Expr::TemporaryIdentifier(id) => Some(match id.facts.binding() {
            Some(key) => Binding::Key(key),
            None => Binding::Temp(id.id),
        }),
        _ => None,
    }
}

fn reads_of(binding: Binding) -> Effects {
    Effects { reads: HashSet::from([binding]), ..Effects::none() }
}

/// `getIntrinsicEffects`: the node's direct effect, excluding its children. Unknown syntax fails closed.
fn intrinsic_effects(node: Node<'_>) -> Effects {
    match node {
        Node::Id(id) => reads_of(binding_of_id(id)),
        Node::StringPart => Effects::none(),
        Node::MapField(m) => match &m.index {
            Expr::String(_) => Effects::none(),
            Expr::Number(n) if rbxts_luau::js_string_to_number(&n.value).is_finite() => Effects::none(),
            _ => Effects::may_throw(),
        },
        Node::Expr(e) => match e {
            Expr::Identifier(_) | Expr::TemporaryIdentifier(_) => {
                reads_of(binding_of_expr(e).expect("an identifier has a binding"))
            }
            Expr::Nil | Expr::True | Expr::False | Expr::Number(_) | Expr::String(_) | Expr::LateString(_) => {
                Effects::none()
            }
            Expr::None | Expr::VarArgs => Effects::none(),
            Expr::Call(n) => {
                let facts = n.expression.facts();
                let known = facts.and_then(|f| f.builtin().or_else(|| f.call_effects()));
                known.map_or_else(Effects::unknown, |e| (**e).clone())
            }
            Expr::MethodCall(_) => Effects::unknown(),
            Expr::Array(_) | Expr::Map(_) | Expr::MixedTable(_) => Effects::allocation(),
            Expr::Set(_) => Effects { throws: true, ..Effects::allocation() },
            Expr::PropertyAccess(_) | Expr::ComputedIndex(_) => {
                if e.facts().is_some_and(|f| f.is_stable_lookup()) { Effects::none() } else { Effects::unknown() }
            }
            Expr::InterpolatedString(n) => {
                let inert = n.parts.iter().all(|p| match p {
                    InterpolatedStringPart::Text(_) => true,
                    InterpolatedStringPart::Expression(e) => is_primitive_value(e),
                });
                if inert { Effects::none() } else { Effects::unknown() }
            }
            Expr::Unary(n) => {
                if n.operator == UnaryOperator::Len && !is_primitive_value(&n.expression) {
                    Effects::unknown()
                } else {
                    Effects::none()
                }
            }
            Expr::Binary(n) => {
                use rbxts_luau::BinaryOperator::{Eq, Ne};
                if matches!(n.operator, Eq | Ne) && !is_primitive_value(&n.left) && !is_primitive_value(&n.right) {
                    Effects::unknown()
                } else {
                    Effects::none()
                }
            }
            Expr::Parenthesized(_) | Expr::If(_) => Effects::none(),
            Expr::Function(_) => Effects::unknown(),
        },
        Node::ElseIf(_) => Effects::none(),
        Node::Stmt(s) => match s {
            Stmt::Assignment(n) => {
                // property writes can invoke __newindex or immediate Instance signal handlers
                let mut effects = Effects::none();
                for target in &n.left {
                    match binding_of_expr(target) {
                        Some(binding) => {
                            effects.writes.insert(binding);
                        }
                        None => effects = join_effects(effects, &Effects::unknown()),
                    }
                }
                effects
            }
            Stmt::VariableDeclaration(n) => {
                let mut effects = Effects::none();
                for target in &n.left {
                    effects.writes.insert(binding_of_id(target));
                }
                effects
            }
            // generalized iteration can call __iter
            Stmt::For(_) => Effects::unknown(),
            // moving an operand into a nonterminating loop can prevent its evaluation
            Stmt::While(_) | Stmt::Repeat(_) => Effects::unknown(),
            // non-finite bounds can fail to terminate
            Stmt::NumericFor(_) => Effects::may_throw(),
            Stmt::Call(_)
            | Stmt::Return(_)
            | Stmt::If(_)
            | Stmt::Do(_)
            | Stmt::Comment(_)
            | Stmt::Break
            | Stmt::Continue => Effects::none(),
            Stmt::FunctionDeclaration(_) | Stmt::MethodDeclaration(_) => Effects::unknown(),
        },
    }
}

fn accumulate(node: Node<'_>, result: &mut Effects) {
    if let Node::Expr(e) = node {
        if is_constant_reference(e) {
            return;
        }
        // creating a closure does not execute its body
        if matches!(e, Expr::Function(_)) {
            result.allocates = true;
            return;
        }
    }
    if let Node::Id(id) = node
        && id.facts().is_constant()
    {
        return;
    }
    let effects = intrinsic_effects(node);
    result.reads.extend(effects.reads);
    result.writes.extend(effects.writes);
    result.reads_heap |= effects.reads_heap;
    result.writes_heap |= effects.writes_heap;
    result.throws |= effects.throws;
    result.allocates |= effects.allocates;
    for_each_child(node, &mut |child| accumulate(child, result));
}

/// Any Luau node the walk visits (`luau.Node`).
#[derive(Clone, Copy)]
enum Node<'a> {
    Expr(&'a Expr),
    Stmt(&'a Stmt),
    /// A declared identifier: a local, a loop variable or a parameter.
    Id(&'a AnyIdentifier),
    /// An `elseif`: an `IfStatement` that is not an element of a statement list.
    ElseIf(&'a IfStatement),
    MapField(&'a MapField),
    /// A literal part of an interpolated string.
    StringPart,
}

fn exprs<'a>(list: &'a [Expr], f: &mut dyn FnMut(Node<'a>)) {
    list.iter().for_each(|e| f(Node::Expr(e)));
}

fn stmts<'a>(list: &'a [Stmt], f: &mut dyn FnMut(Node<'a>)) {
    list.iter().for_each(|s| f(Node::Stmt(s)));
}

fn ids<'a>(list: &'a [AnyIdentifier], f: &mut dyn FnMut(Node<'a>)) {
    list.iter().for_each(|id| f(Node::Id(id)));
}

fn if_children<'a>(node: &'a IfStatement, f: &mut dyn FnMut(Node<'a>)) {
    f(Node::Expr(&node.condition));
    stmts(&node.statements, f);
    match &node.else_body {
        ElseBody::If(else_if) => f(Node::ElseIf(else_if)),
        ElseBody::Block(list) => stmts(list, f),
    }
}

/// `forEachChild` (effects.ts): every child in field order, declared identifiers included.
fn for_each_child<'a>(node: Node<'a>, f: &mut dyn FnMut(Node<'a>)) {
    match node {
        Node::Expr(e) => match e {
            Expr::Identifier(_)
            | Expr::TemporaryIdentifier(_)
            | Expr::None
            | Expr::Nil
            | Expr::False
            | Expr::True
            | Expr::Number(_)
            | Expr::String(_)
            | Expr::LateString(_)
            | Expr::VarArgs => {}
            Expr::ComputedIndex(n) => {
                f(Node::Expr(&n.expression));
                f(Node::Expr(&n.index));
            }
            Expr::PropertyAccess(n) => f(Node::Expr(&n.expression)),
            Expr::Call(n) => {
                f(Node::Expr(&n.expression));
                exprs(&n.args, f);
            }
            Expr::MethodCall(n) => {
                f(Node::Expr(&n.expression));
                exprs(&n.args, f);
            }
            Expr::Parenthesized(n) => f(Node::Expr(&n.expression)),
            Expr::Function(n) => {
                ids(&n.parameters, f);
                stmts(&n.statements, f);
            }
            Expr::Binary(n) => {
                f(Node::Expr(&n.left));
                f(Node::Expr(&n.right));
            }
            Expr::Unary(n) => f(Node::Expr(&n.expression)),
            Expr::If(n) => {
                f(Node::Expr(&n.condition));
                f(Node::Expr(&n.expression));
                f(Node::Expr(&n.alternative));
            }
            Expr::InterpolatedString(n) => {
                for part in &n.parts {
                    match part {
                        InterpolatedStringPart::Text(_) => f(Node::StringPart),
                        InterpolatedStringPart::Expression(e) => f(Node::Expr(e)),
                    }
                }
            }
            Expr::Array(members) | Expr::Set(members) => exprs(members, f),
            Expr::Map(fields) => fields.iter().for_each(|field| f(Node::MapField(field))),
            Expr::MixedTable(fields) => {
                for field in fields {
                    match field {
                        TableField::Value(e) => f(Node::Expr(e)),
                        TableField::Field(m) => f(Node::MapField(m)),
                    }
                }
            }
        },
        Node::Id(_) | Node::StringPart => {}
        Node::MapField(m) => {
            f(Node::Expr(&m.index));
            f(Node::Expr(&m.value));
        }
        Node::ElseIf(n) => if_children(n, f),
        Node::Stmt(s) => match s {
            Stmt::Assignment(n) => {
                exprs(&n.left, f);
                exprs(&n.right, f);
            }
            Stmt::Break | Stmt::Continue | Stmt::Comment(_) => {}
            Stmt::Call(e) => f(Node::Expr(e)),
            Stmt::Do(n) => stmts(&n.statements, f),
            Stmt::While(n) => {
                f(Node::Expr(&n.condition));
                stmts(&n.statements, f);
            }
            Stmt::Repeat(n) => {
                stmts(&n.statements, f);
                f(Node::Expr(&n.condition));
            }
            Stmt::If(n) => if_children(n, f),
            Stmt::NumericFor(n) => {
                f(Node::Id(&n.id));
                f(Node::Expr(&n.start));
                f(Node::Expr(&n.end));
                if let Some(step) = &n.step {
                    f(Node::Expr(step));
                }
                stmts(&n.statements, f);
            }
            Stmt::For(n) => {
                ids(&n.ids, f);
                f(Node::Expr(&n.expression));
                stmts(&n.statements, f);
            }
            Stmt::FunctionDeclaration(n) => {
                f(Node::Expr(&n.name));
                ids(&n.parameters, f);
                stmts(&n.statements, f);
            }
            Stmt::MethodDeclaration(n) => {
                f(Node::Expr(&n.expression));
                ids(&n.parameters, f);
                stmts(&n.statements, f);
            }
            Stmt::VariableDeclaration(n) => {
                ids(&n.left, f);
                exprs(&n.right, f);
            }
            Stmt::Return(n) => exprs(&n.expressions, f),
        },
    }
}
