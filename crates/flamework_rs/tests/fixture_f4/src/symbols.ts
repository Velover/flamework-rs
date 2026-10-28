import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;
declare const sym: unique symbol;
declare const other: unique symbol;

/** Properties TS5's name lookup cannot find again are left out: unique-symbol keys and private names. */
export interface SymbolKeyed {
	b: u8;
	[sym]: u8;
}
export type MappedSymbol = { [K in "b" | typeof sym]: u8 };
export type RecordSymbol = Record<"z" | typeof sym | "a", u8>;
declare class Private {
	#hidden: u8;
	z: u8;
	a: u8;
	[other]: u8;
}
export interface Nested {
	inner: { [sym]: string; keep: u8 };
	list: Array<{ [other]: u8; x: u8 }>;
}
/** Leading underscores are a name like any other. */
export interface Underscored {
	__x: u8;
	___y: u8;
	a: u8;
}

/**
 * A union nothing spells (`Box<A | B>` reaches `value: T`) orders its members by a key of each type, whose object
 * text spells a symbol key as TS5 does (`__@sym@12`, before letters).
 */
interface Box<T> {
	value: T;
}
export type Unspelled = Box<{ [sym]: u8; b: u8 } | { a: u8; c: u8 }>;
export type UnspelledOptional = Box<{ [sym]?: u8; m: u8 } | { n: boolean }>;

export const symbolKeyed = Flamework.createSerializer<SymbolKeyed>({ version: 1 });
export const mappedSymbol = Flamework.createSerializer<MappedSymbol>({ version: 2 });
export const recordSymbol = Flamework.createSerializer<RecordSymbol>();
export const privateNames = Flamework.createSerializer<Private>();
export const nested = Flamework.createSerializer<Nested>();
export const underscored = Flamework.createSerializer<Underscored>();
export const unspelled = Flamework.createSerializer<Unspelled>({ version: 3 });
export const unspelledOptional = Flamework.createSerializer<UnspelledOptional>();
