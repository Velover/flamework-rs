import { Modding, Serialization } from "@flamework-experimental/core";

interface A {
	a: Serialization.u8;
	list: string[] | Set<string> | Map<string, A>;
}
interface B {
	b: string;
	list: A[] | string;
}

/**
 * A serializer macro whose argument can be another one: the inner value is built (and printed) first.
 *
 * @metadata macro
 */
function wrap<T>(inner: unknown, meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>) {
	return [inner, meta];
}

export const nested = wrap<A>(wrap<B>(0));
export const deeper = wrap<A | B>([wrap<B[]>(wrap<Map<string, A>>(1))]);
