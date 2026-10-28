import { Flamework, Modding, Serialization } from "@flamework-experimental/core";
import { Node, two, u8, wrap } from "./argdefs";

// A guard in a written argument ahead of a generated serializer: TS5 names the guard's locals first (print order).
export const w = wrap<Node | Map<string, u8>>(Flamework.createGuard<Node>());

// Generated, written (holding a guard), generated: named in index order.
export const interleaved = two<Node, Map<string, u8>>(undefined, Flamework.createGuard<Node>());

class Wrapper {
	/** @metadata macro */
	wrap<T>(inner: unknown, meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>) {
		return [inner, meta];
	}
}
const wrapper = new Wrapper();
const maybeWrapper = wrapper as Wrapper | undefined;

// The same through a method, `o?.m()` and `super.m()`.
export const viaMethod = wrapper.wrap<Node[]>(Flamework.createGuard<Node>());
export const viaOptional = maybeWrapper?.wrap<Set<Node>>(Flamework.createGuard<Node>());
export class Derived extends Wrapper {
	run() {
		return super.wrap<Map<u8, Node>>(Flamework.createGuard<Node>());
	}
}
