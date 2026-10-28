//! `intrinsic-middleware` (`transformNetworkingMiddlewareIntrinsic`): under obfuscation, the event names of a
//! network's middleware are hashed as the remotes' are, and listed in a seeded order.
//!
//! Without obfuscation the names stay and the order is the written one, so the argument compiles as it is written;
//! the middleware must still be an object literal.

use rbxts_plugin::ts::{Kind, Node, P, Signature, Symbol};
use rbxts_plugin::{Cx, FieldOrder, LuauExpr, ObjectEdits, PResult, synthesized_string};

use crate::plugin::FwInstance;

/// The object literals (config arguments, by parameter index) whose `middleware` names are obfuscated: none without
/// obfuscation. Refuses a `middleware` that is not an object literal, as the TS transformer does either way.
pub(crate) fn targets(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    call: P<Node>,
    signature: P<Signature>,
    parameters: &[P<Symbol>],
) -> PResult<Vec<(usize, P<Node>)>> {
    let obfuscated = fw.shared.config.obfuscation == Some(true);
    let mut targets = Vec::new();
    for &parameter in parameters {
        let index = signature
            .parameters()
            .iter()
            .position(|p| p.value_declaration().and_then(|d| d.symbol()) == Some(parameter));
        let Some(index) = index else { continue };
        let Some(&argument) = call.arguments().get(index) else { continue };
        if argument.kind() != Kind::ObjectLiteralExpression {
            continue;
        }
        let mut found = false;
        for &element in argument.properties() {
            if property_name(element).as_deref() != Some("middleware") {
                continue;
            }
            // `{ middleware }`, `middleware() {}`, `get middleware() {}`: the TS transformer fails an assertion here
            // (it expects a property assignment); the names in them could not be obfuscated.
            match element.kind() {
                Kind::PropertyAssignment => {}
                Kind::ShorthandPropertyAssignment => {
                    return Err(cx.error(element, "Networking middleware must be an object."));
                }
                Kind::MethodDeclaration | Kind::GetAccessor | Kind::SetAccessor => {
                    return Err(cx.error(
                        element,
                        "Networking middleware must be an object, written as `middleware: { ... }`, not as a method or an accessor.",
                    ));
                }
                _ => continue,
            }
            let value = element.initializer().expect("a property assignment has a value");
            if value.kind() != Kind::ObjectLiteralExpression {
                return Err(cx.error(value, "Networking middleware must be an object."));
            }
            found = true;
        }
        if found && obfuscated {
            targets.push((index, argument));
        }
    }
    Ok(targets)
}

/// The config object, its `middleware` obfuscated.
pub(crate) fn compile(fw: &FwInstance, cx: &mut Cx<'_>, call: P<Node>, config: P<Node>) -> PResult<LuauExpr> {
    let mut edits = ObjectEdits::default();
    for &element in config.properties() {
        if element.kind() != Kind::PropertyAssignment || property_name(element).as_deref() != Some("middleware") {
            continue;
        }
        let middleware = element.initializer().expect("a property assignment has a value");
        edits.replace.push(("middleware".to_owned(), obfuscate(fw, cx, call, middleware)?));
    }
    cx.compile_object_literal(config, edits)
}

/// `obfuscateMiddleware`: every named entry renamed to its hash, nested namespaces likewise, in a seeded order.
fn obfuscate(fw: &FwInstance, cx: &mut Cx<'_>, call: P<Node>, object: P<Node>) -> PResult<LuauExpr> {
    let properties = object.properties();
    let mut order: Vec<u32> = (0..properties.len() as u32).collect();
    let mut count = fw.file.borrow().shuffles.get(&call).copied().unwrap_or(0);
    crate::macros::shuffle(fw, cx, call, &mut order, "middleware", &mut count);
    fw.file.borrow_mut().shuffles.insert(call, count);

    let mut edits = ObjectEdits { order: FieldOrder::Permuted(order), ..ObjectEdits::default() };
    for &property in properties {
        if property.kind() != Kind::PropertyAssignment {
            continue;
        }
        let Some(name) = property_name(property) else { continue };
        let hashed = fw.shared.obfuscate_text(&name, Some("remotes"));
        let value = property.initializer().expect("a property assignment has a value");
        if value.kind() == Kind::ObjectLiteralExpression {
            let nested = obfuscate(fw, cx, call, value)?;
            edits.replace.push((name.clone(), nested));
        }
        edits.rename.push((name, synthesized_string(&hashed)));
    }
    cx.compile_object_literal(object, edits)
}

/// A property's name when it has text (`"text" in prop.name`): an identifier, a string or a number.
fn property_name(property: P<Node>) -> Option<String> {
    let name = property.name()?;
    match name.kind() {
        Kind::Identifier | Kind::StringLiteral | Kind::NumericLiteral | Kind::NoSubstitutionTemplateLiteral => {
            Some(name.text().to_owned())
        }
        _ => None,
    }
}
