import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;

interface Base {
	zeta: u8;
	alpha: string;
	mid: boolean;
}

enum Dir {
	Up = "up",
	Down = "down",
	Left = "left",
}

/**
 * Mapped types whose `as` clause renames their keys: their fields go by the key each was made for
 * (`links.keyType`), before the renaming; several keys that name one field go where the first of them would.
 */
export type RemapRev = { [K in "a" | "b" as K extends "a" ? "z" : "y"]: u8 };
export type RemapNum = { [K in 1 | 2 as K extends 1 ? "b" : "a"]: string };
export type RemapMulti = { [K in "a" | "b" as K extends "a" ? "z1" | "z2" : "y"]: u8 };
export type RemapTmpl = { [K in "b" | "a" as `x_${K}`]: u8 };
export type TrueBranch = { [K in "c" | "b" | "a" as K extends "a" | "c" ? K : never]: u8 };
export type Shadowed = { [K in "b" | "a" as K extends infer K ? K : never]: u8 };
export type Excluded = { [K in "c" | "b" | "a" as Exclude<K, "b">]: u8 };
export type Extracted = { [K in 1 | "a" | 2 | "b" as Extract<K, string>]: u8 };
export type Merged = { [K in "c" | "b" | "a" | "e" as K extends "c" ? "a" : K extends "e" ? "d" : K]: u8 };
export type Upper = { [K in "b" | "a" | "c" as Uppercase<K>]: u8 };
export type Numbered = { [K in 10 | 2 | 1 as `n${K}`]: u8 };
export type Directions = { [K in Dir as `d_${K}`]: u8 };
export type Homomorphic = { [K in keyof Base as `get_${K & string}`]: Base[K] };
declare const remapped: RemapRev;
const spreadRemapValue = () => ({ ...remapped, q: 1 as u8 });
export type SpreadRemap = ReturnType<typeof spreadRemapValue>;
export type PartialRemap = Partial<RemapRev>;

export const remapRev = Flamework.createSerializer<RemapRev>({ version: 1 });
export const remapNum = Flamework.createSerializer<RemapNum>({ version: 2 });
export const remapMulti = Flamework.createSerializer<RemapMulti>({ version: 3 });
export const remapTmpl = Flamework.createSerializer<RemapTmpl>({ version: 4 });
export const trueBranch = Flamework.createSerializer<TrueBranch>({ version: 5 });
export const shadowed = Flamework.createSerializer<Shadowed>({ version: 6 });
export const excluded = Flamework.createSerializer<Excluded>({ version: 7 });
export const extracted = Flamework.createSerializer<Extracted>({ version: 8 });
export const merged = Flamework.createSerializer<Merged>({ version: 9 });
export const upper = Flamework.createSerializer<Upper>({ version: 10 });
export const numbered = Flamework.createSerializer<Numbered>({ version: 11 });
export const directions = Flamework.createSerializer<Directions>({ version: 12 });
export const homomorphic = Flamework.createSerializer<Homomorphic>({ version: 13 });
export const spreadRemap = Flamework.createSerializer<SpreadRemap>({ version: 14 });
export const partialRemap = Flamework.createSerializer<PartialRemap>({ version: 15 });
