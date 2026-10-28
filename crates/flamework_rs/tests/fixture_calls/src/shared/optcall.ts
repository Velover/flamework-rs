import { Box, Economy, Save, Shop, macros, sideEffect } from "./macros";

// o.m?.()
export const id0 = macros.id?.<Economy>();
export const id1 = macros.id?.<Economy>(undefined);
export const guard2 = macros.guard?.<Box>();
export const text3 = macros.text?.<Shop | undefined>();
export const dependency4 = macros.dependency?.<Economy>();
export const concise5 = macros.concise?.<Economy>();
export const where6 = macros.where?.();
export const obfuscated7 = macros.obfuscated?.();
export const labels8 = macros.labels?.<[name: string, age: number]>();
export const emit9 = macros.emit?.<{ kind: "x"; list: [1, 2] }>();
export const constant10 = macros.constant?.<{ shared: true }>();
export const path11 = macros.path?.("src/server/commands");
export const glob12 = macros.glob?.("src/server/commands/*.ts");
export const env13 = macros.env?.("F_CALLS_CHANNEL");
export const keys14 = macros.keys?.<{ playerJoined: 1; playerLeft: 2 }>();
export const shuffled15 = macros.shuffled?.<["a", "b", "c"]>();
export const fieldInfo16 = macros.fieldInfo?.<Save>();
export const tupleGuards17 = macros.tupleGuards?.<[string, number]>();
export const serializer18 = macros.serializer?.<Box>();
export const second19 = macros.second?.<Economy>();
export const second20 = macros.second?.<Economy>(sideEffect());
export const withOptions21 = macros.withOptions?.<Economy>({ name: "x" });

export function statements() {
	macros.where?.();
	macros.second?.<Economy>();
	macros.second?.<Economy>(sideEffect());
	return macros.constant?.<{ hoisted: 1 }>();
}
