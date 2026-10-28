import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;
interface Node {
	v: u8;
	kids: Node[];
}

// A guard (F3) ahead of the file's first serializer: its locals are named after the serializers' file-level ones.
export const before = Flamework.createGuard<Node | string>();
export const s = Flamework.createSerializer<u8[] | Map<string, u8> | Set<string>>();
