import { Modding } from "@flamework-experimental/core";

declare const zeta: unique symbol;

/** `Modding.Many<(A | B)[]>` lists a union's values as the call writes it, which is TS5's order for new types. */
type Choice = "zeta" | "alpha" | "mid";
export const aliased = Modding.inspect<Choice[]>();
export const written = Modding.inspect<("zeta" | "alpha")[]>();
export const objects = Modding.inspect<({ z: "x" } | { b: "z" } | { a: "u" })[]>();
export const symbolKeyed = Modding.inspect<({ b: "z" } | { [zeta]: "x"; a: "y" })[]>();
export const tagged = Modding.inspect<({ k: "z" } | { k: "a" })[]>();
