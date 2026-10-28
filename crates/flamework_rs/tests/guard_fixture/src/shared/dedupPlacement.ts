import { Flamework } from "@flamework-experimental/core";

// Dedup locals go ahead of the TS statement around the call (the TS's `prereqList`), not inside the Luau
// expression roblox-ts makes of it.
interface Point {
	x: number;
	y: number;
}

export const lazy = () => Flamework.createGuard<{ a: Point; b: Point; c: Point }>();

export function conditional(flag: boolean) {
	const guard = flag ? Flamework.createGuard<{ a: Point; b: Point; c: Point }>() : undefined;
	return guard;
}

export function shortCircuit(flag: boolean) {
	return flag && Flamework.createGuard<{ a: Point; b: Point; c: Point }>();
}
