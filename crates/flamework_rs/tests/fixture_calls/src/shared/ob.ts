import { Node, u8, wrap } from "./argdefs";

// The macro reached through an object's property (`inner = { wrap }`): a plain call in TS5, no serializer.
const holderObj = { inner: { wrap } };
export const v = holderObj.inner.wrap<Node>(1);
export const after = wrap<Map<string, u8>>(1);
