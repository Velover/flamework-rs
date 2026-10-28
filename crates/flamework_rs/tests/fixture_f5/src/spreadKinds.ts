import { Events } from "./client";

// What a spread argument can be, spread as roblox-ts spreads it: an array whose type is generic, a union of array
// types or read-only (`table.move`), a set's values and a string's characters (a loop each).
export function generic<T extends number[]>(list: T) {
	Events.echo.fire(...list);
}

export function union(list: number[] | [number, number]) {
	Events.echo.fire(1, ...list);
}

export function readonly(list: ReadonlyArray<number>) {
	Events.echo.fire(...list, 2);
}

export function set(values: Set<number>, more: number[]) {
	Events.echo.fire(...values);
	Events.echo.fire(...more, 3, ...values, 4);
}

export function text(word: string) {
	Events.letters.fire(...word);
	Events.letters.fire("a", ...word, "b");
}
