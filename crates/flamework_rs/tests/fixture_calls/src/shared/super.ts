import { Box, Economy, Save, Shop, Macros, macros, sideEffect } from "./macros";

// super.m() and super.m?.()
export class Derived extends Macros {
	plain() {
		const id0 = super.id<Economy>();
		const id1 = super.id<Economy>(undefined);
		const guard2 = super.guard<Box>();
		const text3 = super.text<Shop | undefined>();
		const dependency4 = super.dependency<Economy>();
		const concise5 = super.concise<Economy>();
		const where6 = super.where();
		const obfuscated7 = super.obfuscated();
		const labels8 = super.labels<[name: string, age: number]>();
		const emit9 = super.emit<{ kind: "x"; list: [1, 2] }>();
		const constant10 = super.constant<{ shared: true }>();
		const path11 = super.path("src/server/commands");
		const glob12 = super.glob("src/server/commands/*.ts");
		const env13 = super.env("F_CALLS_CHANNEL");
		const keys14 = super.keys<{ playerJoined: 1; playerLeft: 2 }>();
		const shuffled15 = super.shuffled<["a", "b", "c"]>();
		const fieldInfo16 = super.fieldInfo<Save>();
		const tupleGuards17 = super.tupleGuards<[string, number]>();
		const serializer18 = super.serializer<Box>();
		const second19 = super.second<Economy>();
		const second20 = super.second<Economy>(sideEffect());
		const withOptions21 = super.withOptions<Economy>({ name: "x" });
		const inlined22 = super.inlined<Economy>();
		const inlinedValue23 = super.inlinedValue<Economy>();
		const inlinedValue24 = super.inlinedValue<Economy>(sideEffect());
		const implementsOf25 = super.implementsOf<Shop>(macros);
		const implementsOf26 = super.implementsOf<Shop>(sideEffect());
		return [id0, id1, guard2, text3, dependency4, concise5, where6, obfuscated7, labels8, emit9, constant10, path11, glob12, env13, keys14, shuffled15, fieldInfo16, tupleGuards17, serializer18, second19, second20, withOptions21, inlined22, inlinedValue23, inlinedValue24, implementsOf25, implementsOf26];
	}

	plainStatements() {
		super.where();
		super.second<Economy>();
		super.second<Economy>(sideEffect());
		super.inlined<Economy>();
		super.implementsOf<Shop>(macros);
		super.implementsOf<Shop>(sideEffect());
	}

	optional() {
		const id0 = super.id?.<Economy>();
		const id1 = super.id?.<Economy>(undefined);
		const guard2 = super.guard?.<Box>();
		const text3 = super.text?.<Shop | undefined>();
		const dependency4 = super.dependency?.<Economy>();
		const concise5 = super.concise?.<Economy>();
		const where6 = super.where?.();
		const obfuscated7 = super.obfuscated?.();
		const labels8 = super.labels?.<[name: string, age: number]>();
		const emit9 = super.emit?.<{ kind: "x"; list: [1, 2] }>();
		const constant10 = super.constant?.<{ shared: true }>();
		const path11 = super.path?.("src/server/commands");
		const glob12 = super.glob?.("src/server/commands/*.ts");
		const env13 = super.env?.("F_CALLS_CHANNEL");
		const keys14 = super.keys?.<{ playerJoined: 1; playerLeft: 2 }>();
		const shuffled15 = super.shuffled?.<["a", "b", "c"]>();
		const fieldInfo16 = super.fieldInfo?.<Save>();
		const tupleGuards17 = super.tupleGuards?.<[string, number]>();
		const serializer18 = super.serializer?.<Box>();
		const second19 = super.second?.<Economy>();
		const second20 = super.second?.<Economy>(sideEffect());
		const withOptions21 = super.withOptions?.<Economy>({ name: "x" });
		return [id0, id1, guard2, text3, dependency4, concise5, where6, obfuscated7, labels8, emit9, constant10, path11, glob12, env13, keys14, shuffled15, fieldInfo16, tupleGuards17, serializer18, second19, second20, withOptions21];
	}

	optionalStatements() {
		super.where?.();
		super.second?.<Economy>();
		super.second?.<Economy>(sideEffect());
	}

	/** A generic method: the type argument is the method's own type parameter. */
	generic<T>() {
		return [super.id<T>(), super.text<T>(), super.inlined<T>()];
	}
}
