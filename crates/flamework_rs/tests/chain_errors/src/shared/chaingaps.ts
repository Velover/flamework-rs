import * as ns from "./macros";
import { Economy, Flamework, inlinedText, Macros, macros, maybe, Shop, sideEffect } from "./macros";

// Inlined and rewritten macro calls inside optional chains. TS5 replaces the call node: an inlined call's value
// stands for the whole chain, a rewritten call keeps only its own `?.`. rbxts-rs offers these calls inside the
// chain's nil check, where they cannot behave as in TS5, so flamework-rs refuses each with an error until it offers
// the chain's outermost call first. TS5's output is ts5-reference/chaingaps.luau.

// o.m?.()
export const inlined0_22 = macros.inlined?.<Economy>();
export const inlinedValue0_23 = macros.inlinedValue?.<Economy>();
export const inlinedValue0_24 = macros.inlinedValue?.<Economy>(sideEffect());
export const implementsOf0_25 = macros.implementsOf?.<Shop>(macros);
export const implementsOf0_26 = macros.implementsOf?.<Shop>(sideEffect());

// o?.m()
export const inlined1_22 = maybe?.inlined<Economy>();
export const inlinedValue1_23 = maybe?.inlinedValue<Economy>();
export const inlinedValue1_24 = maybe?.inlinedValue<Economy>(sideEffect());
export const implementsOf1_25 = maybe?.implementsOf<Shop>(macros);
export const implementsOf1_26 = maybe?.implementsOf<Shop>(sideEffect());

// o?.m?.()
export const inlined2_22 = maybe?.inlined?.<Economy>();
export const inlinedValue2_23 = maybe?.inlinedValue?.<Economy>();
export const inlinedValue2_24 = maybe?.inlinedValue?.<Economy>(sideEffect());
export const implementsOf2_25 = maybe?.implementsOf?.<Shop>(macros);
export const implementsOf2_26 = maybe?.implementsOf?.<Shop>(sideEffect());

// super.m?.()
export class DerivedGaps extends Macros {
	optional() {
		return [
			super.inlined?.<Economy>(),
			super.inlinedValue?.<Economy>(),
			super.inlinedValue?.<Economy>(sideEffect()),
			super.implementsOf?.<Shop>(macros),
			super.implementsOf?.<Shop>(sideEffect()),
		];
	}

	statements() {
		super.inlined?.<Economy>();
		super.implementsOf?.<Shop>(sideEffect());
	}
}

// Function macros and core's macros.
export const fnInlined = inlinedText?.<Economy>();
export const nsInlined = ns?.inlinedText<Economy>();
export const coreId = Flamework?.id<Economy>();
export const coreIdCall = Flamework.id?.<Economy>();
export const coreIdBoth = Flamework?.id?.<Economy>();
export const coreEnv = Flamework?.env("F_CALLS_CHANNEL");
export const coreEnvFallback = Flamework.env?.("F_CALLS_MISSING", "fallback");
export const coreImplements = Flamework?.implements<Shop>(macros);
export const coreImplementsCall = Flamework.implements?.<Shop>(sideEffect());

// Inside other expressions and chains.
export const afterRewritten = maybe?.implementsOf<Shop>(sideEffect()) === true;
export const parenthesized = (maybe?.inlined<Economy>())?.size();
export const asArgument = macros.second(maybe?.inlined<Economy>());
export const nested = maybe?.second<Economy>(maybe?.inlined<Shop>());
export const nestedValue = maybe?.inlinedValue<Economy>(maybe?.id<Shop>());

export function statements() {
	maybe?.inlined<Economy>();
	maybe?.implementsOf<Shop>(sideEffect());
	Flamework?.id<Economy>();
	macros.inlined?.<Economy>();
	if (maybe?.inlined<Economy>() !== undefined) {
		return maybe?.text<Shop>();
	}
}
