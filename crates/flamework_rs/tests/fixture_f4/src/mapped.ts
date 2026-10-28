import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;

interface Base {
	zeta: u8;
	alpha: string;
	mid: boolean;
}

/** Mapped types and the types that inherit their properties (spreads and `as` remaps: spreads.ts, remaps.ts). */
export interface ExtPick extends Pick<Base, "zeta" | "alpha"> {
	own: u8;
}
export type InterChain = Pick<Base, "zeta"> & Partial<Pick<Base, "alpha" | "mid">>;
export type OmitPartial = Readonly<Omit<Partial<Base>, "alpha">>;
export type Keyed = Record<"zz" | "aa" | "mm", u8>;
export type Homomorphic = { [K in keyof Base as Exclude<K, "mid">]: Base[K] };
// A generic interface's instance: its mapped bases are instantiated for the argument.
interface ExtGeneric<T extends Base> extends Pick<T, "zeta" | "alpha">, Record<"q" | "b", u8> {
	own: u8;
}
interface ExtGenericDeep<T extends Base> extends ExtGeneric<T> {
	x: u8;
}
// `as` clauses that give each key itself or drop it.
export type Identity = { [K in "b" | "a" as K]: u8 };
export type Filtered = { [K in "d" | "c" | "b" | "a" as K extends "b" ? never : K extends "c" ? never : K]: u8 };

export const extPick = Flamework.createSerializer<ExtPick>({ version: 1 });
export const interChain = Flamework.createSerializer<InterChain>({ version: 2 });
export const omitPartial = Flamework.createSerializer<OmitPartial>({ version: 3 });
export const keyed = Flamework.createSerializer<Keyed>({ version: 4 });
export const homomorphic = Flamework.createSerializer<Homomorphic>({ version: 5 });
export const extGeneric = Flamework.createSerializer<ExtGenericDeep<Base>>({ version: 6 });
export const identity = Flamework.createSerializer<Identity>({ version: 7 });
export const filtered = Flamework.createSerializer<Filtered>({ version: 8 });
