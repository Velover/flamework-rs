//! Key obfuscation (the TS transformer's `expressions/transformAccessExpression.ts`): an access whose receiver's type
//! has `_flamework_key_obfuscation: "<context>"` (networking's remotes) reads the key's obfuscated name, in every
//! position roblox-ts lowers the access in ([`rbxts_plugin::PluginInstance::property_key`]). With obfuscation off
//! the name is the key itself.
//!
//! The marker is looked up on the receiver's non-nullable type, a deliberate difference from the TS transformer, which
//! reads the receiver's type as it is: there an access in an optional chain (`maybe?.k`, `f()?.ns.k`, whose receiver
//! may be `undefined`) keeps its name as written under obfuscation, and a networking call through it reaches no
//! remote. Here it is obfuscated as the same access outside a chain is.
//!
//! As in the TS, only a key written down is known: `obj.k` and `obj["k"]`. `obj[k as "k"]` is left alone (that is
//! how Flamework's own code reaches an obfuscated object), and any other key is an error, whatever its type says.

use rbxts_plugin::ts::Kind;
use rbxts_plugin::{AccessSite, Claim, Cx, PResult};

use crate::macros::string_literal;
use crate::plugin::FwInstance;

/// The marker property, whose string literal type is the obfuscation context.
pub const KEY_OBFUSCATION_MARKER: &str = "_flamework_key_obfuscation";

/// The `property_key` hook for an access whose receiver carries [`KEY_OBFUSCATION_MARKER`].
pub(crate) fn obfuscated_key(fw: &FwInstance, cx: &mut Cx<'_>, access: &AccessSite) -> PResult<Claim<String>> {
    let receiver_type = cx.checker().get_type_at_location(access.receiver);
    // Through `?.` (see the module docs): `T | undefined` carries the marker of `T`.
    let receiver_type = cx.checker().get_non_nullable_type(receiver_type);
    let Some(hash_type) = cx.checker().get_type_of_property_of_type_exported(receiver_type, KEY_OBFUSCATION_MARKER)
    else {
        return Ok(Claim::Decline);
    };
    let Some(context) = string_literal(hash_type) else { return Ok(Claim::Decline) };

    // `getAccessName`: a property name, or a string literal argument.
    let name = match access.node.kind() {
        Kind::PropertyAccessExpression => Some(access.key_node.text()),
        _ if access.key_node.kind() == Kind::StringLiteral => Some(access.key_node.text()),
        _ => None,
    };
    let Some(name) = name else {
        // How obfuscated objects are defined and reached internally.
        if access.node.kind() == Kind::ElementAccessExpression && access.key_node.kind() == Kind::AsExpression {
            return Ok(Claim::Decline);
        }
        return Err(cx.error(access.node, "This object has key obfuscation enabled and must be accessed directly."));
    };
    Ok(Claim::Take(fw.shared.obfuscate_text(name, Some(context))))
}
