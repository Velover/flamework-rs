//! Which calls pack, and for which member (`transformNetworkingCall.ts`: the claim, `packedMember`,
//! `declaredMember`, `markerType`, the refusals of mixed targets). Types only: nothing is compiled, so a hook can
//! still decline after it.

use rbxts_plugin::ts::{Kind, Node, P, Type};
use rbxts_plugin::{Cx, PResult, PluginDiagnostic};
use tsrs_checker::TypeFlags;

use super::mode::{NetworkMode, is_packed, network_mode, serialization_on};
use crate::plugin::FwInstance;

/// Sending methods and the hidden entry point each becomes.
const SENDERS: [(&str, &str); 5] = [
    ("fire", "_fire"),
    ("except", "_except"),
    ("broadcast", "_broadcast"),
    ("invoke", "_invoke"),
    ("invokeWithTimeout", "_invokeWithTimeout"),
];

/// The side of a handler member a call works on, by the hidden marker only that side carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    /// `_flamework_send`: a send.
    Send,
    /// `_flamework_receive`: `setCallback`.
    Receive,
}

impl Side {
    fn marker(self) -> &'static str {
        match self {
            Side::Send => "_flamework_send",
            Side::Receive => "_flamework_receive",
        }
    }
}

/// A call that packs, as the TS transformer transforms it.
#[derive(Clone, Debug)]
pub(crate) enum CallShape {
    /// `target.fire(lead…, values…)` or `target(values…)`: `target._fire(lead…, payload, blobs?)`.
    Send {
        /// The target expression: the method call's receiver, or the callee itself.
        target: P<Node>,
        /// `target` is the call's callee (`handler.event(…)`), not its receiver.
        target_is_callee: bool,
        /// The hidden entry point (`_fire`, `_invoke`, …).
        method: &'static str,
        /// The member's `_flamework_send` list.
        list: P<Type>,
        /// The index of the declared rest parameter: the arguments before it pass through.
        rest_index: usize,
    },
    /// `target.setCallback(cb)`: `target._setCallback(cb, pack)`.
    Callback {
        target: P<Node>,
        /// The receiver's declared member, whose result type the callback's value is packed by.
        function: P<Type>,
    },
}

/// One of the types a call's target may be, and how a call on it is sent.
struct TargetMember {
    ty: P<Type>,
    /// The declared member of the side the call works on.
    function: Option<P<Type>>,
    /// `None` for a value that is not a networking member this kind of call packs for.
    mode: Option<NetworkMode>,
    packed: bool,
}

/// `transformNetworkingCall`'s claim: how `call` packs, or `None` when it is left as it is. `key_of(list, result)` is
/// the serializer's `packingKey`, for a target that may be several members.
/// How a packed member's layout is keyed: its `T` (or callback function type, `true`) to the serializer's packing key.
pub(crate) type KeyOf<'k> = dyn FnMut(&mut Cx<'_>, P<Type>, bool) -> PResult<String> + 'k;

pub(crate) fn analyze(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    call: P<Node>,
    key_of: &mut KeyOf<'_>,
) -> PResult<Option<CallShape>> {
    if call.kind() != Kind::CallExpression {
        return Ok(None);
    }
    let Some(callee) = call.expression() else { return Ok(None) };

    if callee.kind() == Kind::PropertyAccessExpression
        && let Some(name) = callee.name()
    {
        let name = name.text();
        let sender = SENDERS.iter().find(|(method, _)| *method == name).map(|(_, hidden)| *hidden);
        if name == "setCallback" || sender.is_some() {
            let receiver = callee.expression().expect("a property access has an object");
            let target_type = cx.checker().get_type_at_location(receiver);
            let target_type = cx.checker().get_non_nullable_type(target_type);
            let side = if sender.is_some() { Side::Send } else { Side::Receive };
            if let Some((member, function)) = packed_member(fw, cx, target_type, side, receiver, call, key_of)? {
                return Ok(match sender {
                    Some(method) => send_shape(cx, call, receiver, false, member, method),
                    None => callback_shape(call, receiver, function),
                });
            }
        }
    }

    // `handler.event(…)` and `handler.fn(…)`: the call signature is the sender itself.
    let target_type = cx.checker().get_type_at_location(callee);
    let target_type = cx.checker().get_non_nullable_type(target_type);
    let Some((member, _)) = packed_member(fw, cx, target_type, Side::Send, callee, call, key_of)? else {
        return Ok(None);
    };
    let method = if cx.checker().get_property_of_type_exported(member, "_invoke").is_some() { "_invoke" } else { "_fire" };
    Ok(send_shape(cx, call, callee, true, member, method))
}

/// `transformSend`'s checks before it packs: the declared signature has a rest parameter, and the member a list.
fn send_shape(
    cx: &mut Cx<'_>,
    call: P<Node>,
    target: P<Node>,
    target_is_callee: bool,
    member: P<Type>,
    method: &'static str,
) -> Option<CallShape> {
    let signature = cx.checker().get_resolved_signature_exported(call);
    let declaration = signature.declaration()?;
    if !tsrs_ast::is_function_like(Some(declaration)) {
        return None;
    }
    // The declared signature is `(lead…, ...args: I)`; the resolved one has `I` expanded.
    let rest_index = declaration.parameters().iter().position(|p| p.as_parameter_declaration().dot_dot_dot_token().is_some())?;
    let list = marker_type(cx, member, "_flamework_send", call)?;
    Some(CallShape::Send { target, target_is_callee, method, list, rest_index })
}

/// `transformReceiverCallback`'s check: a callback argument.
fn callback_shape(call: P<Node>, target: P<Node>, function: P<Type>) -> Option<CallShape> {
    call.arguments().first()?;
    Some(CallShape::Callback { target, function })
}

/// `packedMember`: the handler member a call on `target_type` packs for, with its side's declared member, or `None`
/// when the call is left as it is. A union of members packs only when every member does, and then as the first of
/// them; members packed differently, or packed members whose lists are not laid out and checked alike, are refused.
fn packed_member(
    fw: &FwInstance,
    cx: &mut Cx<'_>,
    target_type: P<Type>,
    side: Side,
    target: P<Node>,
    call: P<Node>,
    key_of: &mut KeyOf<'_>,
) -> PResult<Option<(P<Type>, P<Type>)>> {
    let target_name = cx.source_text(target).to_owned();
    let resolved = if target_type.flags().intersects(TypeFlags::Instantiable) {
        cx.checker().get_apparent_type_exported(target_type)
    } else {
        target_type
    };
    let types: Vec<P<Type>> =
        if resolved.flags().intersects(TypeFlags::Union) { resolved.types().to_vec() } else { vec![resolved] };

    let mut members = Vec::with_capacity(types.len());
    for ty in types {
        if let Some(function) = declared_member(cx, ty, side, call) {
            let mode = network_mode(cx, Some(function), call, Some(&target_name))?;
            members.push(TargetMember { ty, function: Some(function), mode: Some(mode), packed: is_packed(fw, mode) });
        } else {
            // A raw member carries `_flamework_packing` without the marker.
            let raw = cx.checker().get_property_of_type_exported(ty, super::PACKING_MARKER).is_some();
            members.push(TargetMember { ty, function: None, mode: raw.then_some(NetworkMode::Raw), packed: false });
        }
    }

    let packed: Vec<&TargetMember> = members.iter().filter(|m| m.packed).collect();
    if packed.is_empty() {
        return Ok(None);
    }
    let is_callback = side == Side::Receive;
    if packed.len() < members.len() {
        let modes = describe_modes(fw, &members);
        let consequence = if is_callback {
            "One call site packs the results of all of them or of none, so the callers of some would reject what the callback returns."
        } else {
            "One call site packs for all of them or for none, so some would drop what it sends as malformed."
        };
        return Err(refuse_mixed_target(cx, call, &modes, consequence));
    }

    // Several members that all pack: the one encoding has to suit each of them, so their layouts are compared.
    if packed.len() > 1 {
        let mut keyed = Vec::with_capacity(packed.len());
        for member in &packed {
            let function = member.function.expect("a packed member has its declared member");
            let carried = if is_callback { Some(function) } else { marker_type(cx, member.ty, "_flamework_send", call) };
            let key = match carried {
                Some(carried) => Some(key_of(cx, carried, is_callback)?),
                None => None,
            };
            keyed.push((key, member_text(cx, member.ty, function)));
        }
        keyed.sort_by(|a, b| crate::js::utf16_cmp(&a.1, &b.1));
        if let Some(other) = keyed.iter().find(|(key, _)| key.is_none() || *key != keyed[0].0) {
            let what = if is_callback { "results" } else { "argument lists" };
            let consequence = if is_callback {
                "One call site packs the callback's results one way, so the callers of the other would read them wrong."
            } else {
                "One call site packs one way, so the receiver of the other would read what it sends wrong."
            };
            let message = format!("their {what} are not laid out and checked alike ('{}' and '{}')", keyed[0].1, other.1);
            return Err(refuse_mixed_target(cx, call, &message, consequence));
        }
    }

    let first = packed[0];
    Ok(Some((first.ty, first.function.expect("a packed member has its declared member"))))
}

/// `declaredMember`: the declared member (`_flamework_fn`) of the side of `ty` a call works on. Only the part of an
/// intersection (a name declared in both directions) that carries the side's own marker is read.
fn declared_member(cx: &mut Cx<'_>, ty: P<Type>, side: Side, node: P<Node>) -> Option<P<Type>> {
    let parts: Vec<P<Type>> =
        if ty.flags().intersects(TypeFlags::Intersection) { ty.types().to_vec() } else { vec![ty] };
    for part in parts {
        if cx.checker().get_property_of_type_exported(part, side.marker()).is_none() {
            continue;
        }
        let function = marker_type(cx, part, "_flamework_fn", node);
        // A sender of networking 2.0.0-alpha.3 or earlier carries no declared member: `any`, a plain member.
        if function.is_some() || side == Side::Send {
            return Some(function.unwrap_or_else(|| cx.checker().get_any_type()));
        }
    }
    None
}

/// `markerType`: the type a hidden marker property carries, without the `undefined` its optionality adds.
pub(crate) fn marker_type(cx: &mut Cx<'_>, ty: P<Type>, marker: &str, node: P<Node>) -> Option<P<Type>> {
    let property = cx.checker().get_property_of_type_exported(ty, marker)?;
    let marker_type = cx.checker().get_type_of_symbol_at_location(property, Some(node))?;
    Some(cx.checker().get_non_nullable_type(marker_type))
}

/// `memberText`: a member as declared, for a message (`ServerEvents.move(x: string | number): void`).
fn member_text(cx: &mut Cx<'_>, member: P<Type>, function: P<Type>) -> String {
    let signature = cx
        .checker()
        .get_signatures_of_type(function, tsrs_checker::SignatureKind::Call)
        .first()
        .copied();
    let declaration = signature.and_then(|s| s.declaration()).filter(|&d| tsrs_ast::is_function_like(Some(d)));
    let Some(declaration) = declaration else { return cx.checker().type_to_string_exported(member) };

    let mut owner = declaration.parent();
    while let Some(node) = owner {
        if matches!(node.kind(), Kind::MethodSignature | Kind::PropertySignature | Kind::TypeAliasDeclaration) {
            break;
        }
        owner = node.parent();
    }
    if declaration.kind() == Kind::MethodSignature {
        owner = Some(declaration);
    }
    let mut name = String::new();
    if let Some(owner) = owner {
        if owner.kind() == Kind::TypeAliasDeclaration {
            name = owner.name().map(|n| n.text().to_owned()).unwrap_or_default();
        } else {
            let container = owner
                .parent()
                .filter(|p| p.kind() == Kind::InterfaceDeclaration)
                .and_then(|p| p.name())
                .map(|n| format!("{}.", n.text()))
                .unwrap_or_default();
            let own = owner.name().map(|n| cx.source_text(n)).unwrap_or_default();
            name = format!("{container}{own}");
        }
    }
    let parameters: Vec<&str> = declaration.parameters().iter().map(|&p| cx.source_text(p)).collect();
    let result = declaration.type_node().map(|t| format!(": {}", cx.source_text(t))).unwrap_or_default();
    collapse_whitespace(&format!("{name}({}){result}", parameters.join(", ")))
}

/// `text.replace(/\s+/g, " ")`.
pub(crate) fn collapse_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut space = false;
    for c in text.chars() {
        if crate::js::is_whitespace(c) || crate::js::is_line_terminator(c) {
            space = true;
            continue;
        }
        if space {
            out.push(' ');
            space = false;
        }
        out.push(c);
    }
    if space {
        out.push(' ');
    }
    out
}

/// `describeModes`: how the members of a mixed target are sent, one phrase per way.
fn describe_modes(fw: &FwInstance, members: &[TargetMember]) -> String {
    let switch_on = serialization_on(fw);
    let mut phrases = Vec::new();
    let has = |mode: Option<NetworkMode>| members.iter().any(|m| m.mode == mode);
    if has(Some(NetworkMode::Serialized)) {
        phrases.push("Serialized members are packed into a buffer");
    }
    if has(Some(NetworkMode::Plain)) {
        phrases.push(if switch_on {
            "plain members are packed, as networking.serialization is on"
        } else {
            "plain members are sent as they are, as networking.serialization is off"
        });
    }
    if has(Some(NetworkMode::Raw)) {
        phrases.push("Raw members are sent as they are");
    }
    if has(None) {
        phrases.push("a value that is not a networking member is called as it is");
    }
    phrases.join("; ")
}

/// `refuseMixedTarget`: the build error for a call whose target may be members one call site cannot pack for.
fn refuse_mixed_target(cx: &mut Cx<'_>, call: P<Node>, what: &str, consequence: &str) -> rbxts_plugin::Abort {
    let text = call.expression().map(|callee| collapse_whitespace(cx.source_text(callee)));
    let subject = match text {
        Some(text) => format!("The call '{text}(...)'"),
        None => "This call".to_owned(),
    };
    let diagnostic = PluginDiagnostic::error(
        call,
        format!("{subject} may reach networking members that are packed differently: {what}."),
    )
    .with_hint(format!(
        "{consequence}\nMake the call where the member's own type is known, such as in each case of a switch over the name, or declare these members the same way."
    ));
    cx.report(diagnostic).expect("an error")
}
