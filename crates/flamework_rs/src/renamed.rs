//! `explainRenamed` (`util/functions/explainRenamed.ts`): TypeScript's error for a name Flamework renamed or removed
//! before beta, as one that names the replacement. Asked by the compiler for the checker diagnostics of a file that
//! fails checking, which the plugin forwards as the TS transformer does ([`crate::plugin::Flamework`]).

use rbxts_plugin::ts::{Diagnostic, Kind, Node, P, Symbol};
use rbxts_plugin::{Cx, PluginDiagnostic};
use tsrs_ast::SymbolFlags;
use tsrs_checker::ContextFlags;

use crate::packages::{COMPONENTS_PACKAGE, CORE_PACKAGE};

/// A name dropped before beta: the name that replaces it (on the same type or namespace), the package that declares
/// the replacement (only its own declarations are explained), and whether it was removed (worded differently) rather
/// than renamed.
struct Rename {
    old: &'static str,
    replacement: &'static str,
    package: &'static str,
    removed: bool,
}

/// `RENAMES`.
const RENAMES: [Rename; 5] = [
    Rename { old: "onPostIgnite", replacement: "onConstructed", package: CORE_PACKAGE, removed: false },
    Rename { old: "createPlugin", replacement: "builder", package: COMPONENTS_PACKAGE, removed: false },
    Rename { old: "ancestorWhitelist", replacement: "ancestorAllowList", package: COMPONENTS_PACKAGE, removed: false },
    Rename { old: "ancestorBlacklist", replacement: "ancestorBlockList", package: COMPONENTS_PACKAGE, removed: false },
    Rename { old: "Default", replacement: "Contextual", package: COMPONENTS_PACKAGE, removed: true },
];

/// TypeScript's codes for a name a type or a namespace does not have (`MISSING_NAME_CODES`): a property (2339, 2551),
/// an object literal's key (2353, 2561), a namespace's member (2694, 2724).
const MISSING_NAME_CODES: [i32; 6] = [2339, 2551, 2353, 2561, 2694, 2724];

/// `explainRenamed`: the explained diagnostic, or `None` for any other diagnostic and where the replacement is not on
/// the same type or namespace, declared by Flamework's package.
pub(crate) fn explain_renamed(cx: &mut Cx<'_>, diagnostic: P<Diagnostic>) -> Option<PluginDiagnostic> {
    if !MISSING_NAME_CODES.contains(&diagnostic.code()) {
        return None;
    }
    let file = diagnostic.file()?;
    let name = crate::networking::mode::identifier_at(file.as_node(), diagnostic.pos()?)?;
    let rename = RENAMES.iter().find(|rename| rename.old == name.text())?;

    let replacement = find_replacement(cx, name, rename.replacement)?;
    let by_flamework = replacement
        .declarations()
        .iter()
        .any(|&declaration| crate::class_hook::declaring_package(cx, declaration).as_deref() == Some(rename.package));
    if !by_flamework {
        return None;
    }

    let parent = name.parent()?;
    let owner = match parent.kind() {
        Kind::PropertyAccessExpression if parent.as_property_access_expression().name == name => {
            format!("{}.", cx.source_text(parent.as_property_access_expression().expression))
        }
        Kind::QualifiedName if parent.as_qualified_name().right == name => {
            format!("{}.", cx.source_text(parent.as_qualified_name().left))
        }
        _ => String::new(),
    };
    let old = format!("{owner}{}", name.text());
    let new = format!("{owner}{}", rename.replacement);
    let message = if rename.removed {
        format!("'{old}' was removed before beta. It stood for '{new}', which replaces it.")
    } else {
        format!("'{old}' was renamed '{new}' before beta.")
    };
    Some(PluginDiagnostic::error(name, message).with_hint(format!("Write '{}'.", rename.replacement)))
}

/// `findReplacement`: the symbol named `replacement` on whatever `name` was looked up on, if it has one.
fn find_replacement(cx: &mut Cx<'_>, name: P<Node>, replacement: &str) -> Option<P<Symbol>> {
    let parent = name.parent()?;
    match parent.kind() {
        Kind::PropertyAccessExpression if parent.as_property_access_expression().name == name => {
            let ty = cx.checker().get_type_at_location(parent.as_property_access_expression().expression);
            cx.checker().get_property_of_type_exported(ty, replacement)
        }
        Kind::QualifiedName if parent.as_qualified_name().right == name => {
            let mut namespace = cx.checker().get_symbol_at_location_exported(parent.as_qualified_name().left)?;
            if namespace.flags().intersects(SymbolFlags::Alias) {
                namespace = cx.checker().get_aliased_symbol(namespace);
            }
            namespace.exports()?.lookup(replacement)
        }
        Kind::PropertyAssignment | Kind::ShorthandPropertyAssignment
            if parent.name() == Some(name)
                && parent.parent().is_some_and(|literal| literal.kind() == Kind::ObjectLiteralExpression) =>
        {
            let literal = parent.parent()?;
            let contextual = cx.checker().get_contextual_type_exported(literal, ContextFlags::None)?;
            let members = if contextual.is_union() { contextual.types().to_vec() } else { vec![contextual] };
            members.into_iter().find_map(|ty| cx.checker().get_property_of_type_exported(ty, replacement))
        }
        _ => None,
    }
}
