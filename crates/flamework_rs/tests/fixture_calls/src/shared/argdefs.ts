import { Modding, Serialization } from "@flamework-experimental/core";

export type u8 = Serialization.u8;
export interface A {
	a: u8;
	list: string[] | Set<string> | Map<string, A>;
}
export interface B {
	b: string;
	list: A[] | string;
}
export interface C {
	c: Enum.Material;
	d: Map<string, u8> | u8[];
}
export interface Node {
	v: u8;
	kids: Node[];
}

/** @metadata macro */
export function wrap<T>(inner: unknown, meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>) {
	return [inner, meta];
}

/** @metadata macro */
export function pre<T>(meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>, inner?: unknown) {
	return [meta, inner];
}

/** @metadata macro */
export function two<T, U>(
	first?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>,
	written?: unknown,
	second?: Modding.Intrinsic<"serializer", [U], Serialization.Serializer<U>>,
) {
	return [first, written, second];
}
