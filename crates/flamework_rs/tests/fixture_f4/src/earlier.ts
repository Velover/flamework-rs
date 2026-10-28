import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;
interface Node {
	v: u8;
	kids: Node[];
}

// A guard (F3) earlier in the root statement than a serializer: what the serializer hoisted printed ahead of the
// statement, so it is named first.
export const both = [Flamework.createGuard<Node>(), Flamework.createSerializer<Node | Map<string, u8>>()];
