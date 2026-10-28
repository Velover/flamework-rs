import * as ns from "./macros";
import { Box, Economy, Flamework, holder, idOf, macros, maybe } from "./macros";

// Function macros: `f?.()`, `ns?.f()`, `ns.f?.()`.
export const fnOptional = idOf?.<Economy>();
export const nsReceiver = ns?.idOf<Economy>();
export const nsCall = ns.idOf?.<Economy>();

// Core's macros through optional chains (generated, not inlined or rewritten).
export const coreGuard = Flamework?.createGuard<Box>();
export const coreGuardCall = Flamework.createGuard?.<Box>();

// Longer chains: the macro call after more links, links after the macro call, and nested optional chains.
export const deep = holder?.macros?.id<Economy>();
export const deepBoth = holder?.macros?.id?.<Economy>();
export const deepPlain = holder?.macros!.second<Economy>(macros.text<Box>());
export const after = maybe?.id<Economy>().size();
export const coalesced = maybe?.id<Economy>() ?? "none";
export const inlinedFirst = macros.inlined<Economy>()?.size();
export const nestedArgument = maybe?.second<Economy>(maybe?.text<Box>());

// Callees TypeScript's `getSymbolAtLocation` has no symbol for: not macros in TS5, plain calls.
export const elementCall = maybe?.["id"]<Economy>();
export const elementPlain = macros["id"]<Economy>();
export const elementBoth = maybe?.["id"]?.<Economy>();
export const elementOptionalId = macros["id"]?.<Economy>();
export const elementOptional = macros["inlined"]?.<Economy>();
export const parenthesizedCallee = (macros.id)<Economy>();
export const nonNullCallee = macros.id!<Economy>();

export function statements() {
	holder?.macros?.where();
	maybe?.second<Economy>();
	if (maybe?.id<Economy>() !== undefined) {
		return maybe?.guard<Box>();
	}
}
