import { Box, Economy, Save, Shop, macros, maybe, sideEffect } from "./macros";

// o?.m?.()
export const id0 = maybe?.id?.<Economy>();
export const id1 = maybe?.id?.<Economy>(undefined);
export const guard2 = maybe?.guard?.<Box>();
export const text3 = maybe?.text?.<Shop | undefined>();
export const dependency4 = maybe?.dependency?.<Economy>();
export const concise5 = maybe?.concise?.<Economy>();
export const where6 = maybe?.where?.();
export const obfuscated7 = maybe?.obfuscated?.();
export const labels8 = maybe?.labels?.<[name: string, age: number]>();
export const emit9 = maybe?.emit?.<{ kind: "x"; list: [1, 2] }>();
export const constant10 = maybe?.constant?.<{ shared: true }>();
export const path11 = maybe?.path?.("src/server/commands");
export const glob12 = maybe?.glob?.("src/server/commands/*.ts");
export const env13 = maybe?.env?.("F_CALLS_CHANNEL");
export const keys14 = maybe?.keys?.<{ playerJoined: 1; playerLeft: 2 }>();
export const shuffled15 = maybe?.shuffled?.<["a", "b", "c"]>();
export const fieldInfo16 = maybe?.fieldInfo?.<Save>();
export const tupleGuards17 = maybe?.tupleGuards?.<[string, number]>();
export const serializer18 = maybe?.serializer?.<Box>();
export const second19 = maybe?.second?.<Economy>();
export const second20 = maybe?.second?.<Economy>(sideEffect());
export const withOptions21 = maybe?.withOptions?.<Economy>({ name: "x" });

export function statements() {
	maybe?.where?.();
	maybe?.second?.<Economy>();
	maybe?.second?.<Economy>(sideEffect());
	return maybe?.constant?.<{ hoisted: 1 }>();
}
