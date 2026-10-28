import { Flamework } from "@flamework-experimental/core";
import { Node, u8, wrap } from "./argdefs";

// Calls TS5 leaves alone after a serializer was made: theirs must take no names (a callee holding a call, a property
// holding the macro).
function getWrap() {
	return wrap;
}
const holderObj = { inner: { wrap } };
export const first = wrap<Node>(1);
export const c1 = getWrap()<Node>(Flamework.createGuard<Node>());
export const c2 = holderObj.inner.wrap<Node[]>(Flamework.createGuard<Node>());
export const after = wrap<Map<string, u8>>(Flamework.createGuard<Node>());
