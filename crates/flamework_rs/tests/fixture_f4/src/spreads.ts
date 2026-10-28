import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;

interface Base {
	zeta: u8;
	alpha: string;
	mid: boolean;
}

/**
 * Properties a spread or an object rest copied out of a mapped type: each keeps its link to the mapped type that made
 * it (`links.mappedType`), and its run of fields goes in that type's order.
 */
declare const picked: Pick<Base, "zeta" | "alpha">;
const spreadPickValue = () => ({ first: 1 as u8, ...picked });
export type SpreadPick = ReturnType<typeof spreadPickValue>;
declare const recorded: Record<"zz" | "aa", u8>;
const spreadRecValue = () => ({ ...recorded, k: true });
export type SpreadRec = ReturnType<typeof spreadRecValue>;
declare const omitted: Omit<Base, "mid">;
const spreadOmitValue = () => ({ ...omitted, extra: 1 as u8 });
export type SpreadOmit = ReturnType<typeof spreadOmitValue>;
declare const chained: Partial<Pick<Base, "zeta" | "alpha">>;
const spreadChainValue = () => ({ ...chained, n: 2 as u8 });
export type SpreadChain = ReturnType<typeof spreadChainValue>;
// Two mapped runs with a written property between them.
declare const keyed: Record<"q" | "c", u8>;
const twoRunsValue = () => ({ a1: 1 as u8, ...keyed, z9: true, ...picked });
export type TwoRuns = ReturnType<typeof twoRunsValue>;
// Object rests of one run (properties one mapped type made, a single property): their order does not depend on the
// order the rest lists them in, which rbxts-rs does not rebuild yet (a rest of several runs is refused).
declare const four: Record<"c" | "b" | "a" | "d", u8>;
const restValue = () => {
	const { b: _b, ...rest } = four;
	return rest;
};
export type Rest = ReturnType<typeof restValue>;
interface Two {
	b: u8;
	a: string;
}
declare const two: Two;
const oneLeftValue = () => {
	const { a: _a, ...rest } = two;
	return rest;
};
export type OneLeft = ReturnType<typeof oneLeftValue>;
// A spread of a rest of one run: the rest's properties keep their block, in the mapped type's order.
declare const restOfFour: Rest;
const fromRestValue = () => ({ k: true, ...restOfFour, z: 1 as u8 });
export type FromRest = ReturnType<typeof fromRestValue>;
// A generic interface's instance: its mapped bases are instantiated for the argument.
interface Extended<T extends Base> extends Pick<T, "zeta" | "mid">, Record<"y" | "b", u8> {
	own: u8;
}
declare const extended: Extended<Base>;
const spreadExtendedValue = () => ({ ...extended, z: 1 as u8 });
export type SpreadExtended = ReturnType<typeof spreadExtendedValue>;
export type PartialSpread = Partial<SpreadPick>;

export const spreadPick = Flamework.createSerializer<SpreadPick>({ version: 1 });
export const spreadRec = Flamework.createSerializer<SpreadRec>({ version: 2 });
export const spreadOmit = Flamework.createSerializer<SpreadOmit>({ version: 3 });
export const spreadChain = Flamework.createSerializer<SpreadChain>({ version: 4 });
export const twoRuns = Flamework.createSerializer<TwoRuns>({ version: 5 });
export const rest = Flamework.createSerializer<Rest>({ version: 6 });
export const spreadExtended = Flamework.createSerializer<SpreadExtended>({ version: 7 });
export const generic = Flamework.createSerializer<Extended<Base>>({ version: 8 });
export const partialSpread = Flamework.createSerializer<PartialSpread>({ version: 9 });
export const inArray = Flamework.createSerializer<{ list: SpreadOmit[]; map: Map<string, SpreadRec> }>({ version: 10 });
export const oneLeft = Flamework.createSerializer<{ left: OneLeft; rest: Rest }>({ version: 11 });
export const fromRest = Flamework.createSerializer<FromRest>({ version: 12 });
