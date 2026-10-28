import { Flamework } from "@flamework-experimental/core";
import { Node, u8, wrap } from "./argdefs";

// A callee whose own symbol is not the macro (an alias of it): TS5 leaves the call alone, so no serializer is made
// for it. Here it would have been the file's first, and the codec's preamble comes with the first one made.
const alias = wrap;
export const v = alias<Node>(Flamework.createGuard<Map<string, u8>>());
export const after = wrap<Map<string, u8>>(1);
