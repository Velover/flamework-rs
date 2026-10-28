import { Events } from "./client";

// An optional chain: the compiler's nil check holds the packing, where TS5 wraps the call in a function that checks
// the chain's start (one of the two files of the fixture that differ from TS5's bytes; README.md, Known limitations).
export function chained(maybe: typeof Events | undefined, value: number) {
	maybe?.alpha.fire(value * 3);
	maybe?.charlie.fire();
}
