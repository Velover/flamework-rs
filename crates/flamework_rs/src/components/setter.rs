//! Attribute setters (the TS transformer's `expressions/transformBinaryExpression.ts`, `transformUnaryExpression.ts`,
//! `transformDeleteExpression.ts` and `util/functions/isAttributesAccess.ts`): a write to `x.attributes.<name>`,
//! where `attributes` is declared with `@metadata intrinsic-component-attributes`, goes through the component, which
//! writes the value back to the instance:
//!
//! - `x.attributes.k = v` → `x[SYMBOL_ATTRIBUTE_SETTER](x, "k", v)`;
//! - `x.attributes.k op= v` → the value `x.attributes.k op v`, lowered by roblox-ts on the written operands
//!   ([`Cx::compile_binary`]: a string `+` stays a concatenation, `??` its own lowering);
//! - `++` / `--` → `x.attributes.k ± 1`, with a fourth argument `true` for the postfix forms (always, as the TS);
//! - `delete x.attributes.k` → the value `nil`.
//!
//! `SYMBOL_ATTRIBUTE_SETTER` is imported from components' `out/baseComponent`. The call is the one roblox-ts makes of
//! the TS's `x[SETTER](name, value)` (`transformElementCallExpressionInner`, `ensureTransformOrder`,
//! `createOrderedPropertyCall`), with roblox-ts's effect analysis ([`super::effects`]) deciding what goes in `_exp`,
//! `_self` and `_fn` locals, so it evaluates (and reads) as TS 5's.

use rbxts_luau::{BinaryOperator, Effects, create};
use rbxts_plugin::ts::{Kind, Node, P};
use rbxts_plugin::{AssignOp, AssignSite, Claim, Cx, LuauExpr, PResult, synthesized_string};

use super::effects::{effects_commute, get_effects, get_effects_of_statements, is_late_read, join_effects};
use crate::packages::COMPONENTS_PACKAGE;

/// The module `SYMBOL_ATTRIBUTE_SETTER` is imported from.
fn setter_module() -> String {
    format!("{COMPONENTS_PACKAGE}/out/baseComponent")
}

/// The target as the TS sees it: the node's own operand (the TS does not look through parentheses).
fn written_target(site: &AssignSite) -> Option<P<Node>> {
    let node = site.node;
    Some(match node.kind() {
        Kind::BinaryExpression => node.as_binary_expression().left,
        Kind::PrefixUnaryExpression => node.as_prefix_unary_expression().operand,
        Kind::PostfixUnaryExpression => node.as_postfix_unary_expression().operand,
        Kind::DeleteExpression => node.expression()?,
        _ => return None,
    })
}

/// The assignment hook for a target whose receiver is a component's `attributes`.
pub(crate) fn attribute_setter(cx: &mut Cx<'_>, site: &AssignSite) -> PResult<Claim<LuauExpr>> {
    // `isAttributesAccess(state, node.left)`: the written target is an access whose receiver has the symbol. The TS
    // asks `getSymbolAtLocation` of the receiver as written, which knows names and property accesses only (not
    // parentheses, casts or element accesses).
    if written_target(site) != Some(site.target)
        || site.target.expression() != Some(site.receiver)
        || !matches!(site.receiver.kind(), Kind::Identifier | Kind::PropertyAccessExpression)
    {
        return Ok(Claim::Decline);
    }
    if site.receiver.kind() != Kind::PropertyAccessExpression {
        return Err(cx.error(site.target, "assignments not supported with direct access"));
    }
    let Some(this_access) = site.receiver.expression() else { return Ok(Claim::Decline) };

    // The TS's `x[SYMBOL_ATTRIBUTE_SETTER](name, value)`, compiled as roblox-ts compiles an element call
    // (`transformElementCallExpressionInner`), with its effect analysis ([`super::effects`]) deciding what is held.
    let mut setter = cx.import_named(&setter_module(), "SYMBOL_ATTRIBUTE_SETTER")?;
    // An import is a constant reference (`markBinding`).
    if let Some(facts) = setter.facts_mut() {
        facts.set_constant(true);
    }
    let mut base = cx.compile_expr(this_access)?;
    // A late-read receiver is held before a key with effects (the key is the import: never).
    let later = if is_late_read(&base) { get_effects(&setter) } else { Effects::none() };
    if !effects_commute(&get_effects(&base), &later) {
        base = cx.push_to_var(base, "exp");
    }

    // The arguments (`ensureTransformOrder`): each compiled with the statements it needs, the name held when the
    // value's statements could change it. Deliberate deviation: an element key is compiled with Flamework's hooks, so
    // an attribute write or a macro inside it is transformed. The TS transformer passes `getIndexExpression` on
    // untransformed (transformBinaryExpression.ts, transformUnaryExpression.ts, transformDeleteExpression.ts), which
    // writes the attributes table without the setter and leaves the macro unexpanded: a bug there, not copied here.
    let (name, name_statements) = cx.capture(|cx| match site.target.kind() {
        Kind::PropertyAccessExpression => Ok(synthesized_string(site.key_node.text())),
        _ => cx.compile_expr(site.key_node),
    })?;
    let (value, value_statements) = cx.capture(|cx| match site.op {
        AssignOp::Assign => match site.value {
            Some(value) => cx.compile_expr(value),
            None => Ok(create::nil()),
        },
        AssignOp::Compound(op) => match site.value {
            Some(value) => cx.compile_binary(op, site.target, value),
            None => Ok(create::nil()),
        },
        AssignOp::PreInc | AssignOp::PostInc => {
            Ok(create::binary(cx.compile_expr(site.target)?, BinaryOperator::Add, create::number(1.0)))
        }
        AssignOp::PreDec | AssignOp::PostDec => {
            Ok(create::binary(cx.compile_expr(site.target)?, BinaryOperator::Sub, create::number(1.0)))
        }
        AssignOp::Delete => Ok(create::nil()),
    })?;
    let capture_name = !effects_commute(&get_effects(&name), &get_effects_of_statements(&value_statements));
    let mut statements = name_statements;
    let name = if capture_name {
        let (name, held) = cx.capture(|cx| Ok(cx.push_to_var(name, "exp")))?;
        statements.extend(held);
        name
    } else {
        name
    };
    statements.extend(value_statements);

    // `createOrderedPropertyCall`: an explicit self argument denotes the receiver from before the lookup, and the
    // lookup may not move past the arguments' statements.
    let prereq_effects = get_effects_of_statements(&statements);
    let mut callee = create::computed_index(base.clone(), setter.clone());
    let lookup_effects = get_effects(&callee);
    let capture_callee = !effects_commute(&lookup_effects, &prereq_effects);
    if !base.is_simple() || !effects_commute(&get_effects(&base), &join_effects(lookup_effects, &prereq_effects)) {
        base = cx.push_to_var(base, "self");
        callee = create::computed_index(base.clone(), setter);
    }
    if capture_callee {
        callee = cx.push_to_var(callee, "fn");
    }
    for statement in statements {
        cx.prereq(statement);
    }

    let mut args = vec![base, name, value];
    if matches!(site.op, AssignOp::PostInc | AssignOp::PostDec) {
        args.push(create::bool(true));
    }
    Ok(Claim::Take(create::call(callee, args)))
}
