import { Events } from "./client";

// An optional chain: in a function that checks the chain's start, packs, and then reads the chain again for the send,
// as TS5 does.
export function chained(maybe: typeof Events | undefined, value: number) {
	maybe?.alpha.fire(value * 3);
	maybe?.charlie.fire();
}
