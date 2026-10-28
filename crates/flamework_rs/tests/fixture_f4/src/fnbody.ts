import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;
interface Node {
	v: u8;
	kids: Node[];
}

// The same in a function body: one root statement, the guard's statement first.
export function make() {
	const g = Flamework.createGuard<Node>();
	const s = Flamework.createSerializer<Node | Map<string, u8>>();
	return [g, s];
}
