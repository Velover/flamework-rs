import { Flamework, Serialization } from "@flamework-experimental/core";

/** Numbers at every width, strict and implicit, and the varint. */
export const f64 = Flamework.createSerializer<number>();
export const u8 = Flamework.createSerializer<Serialization.u8>();
export const i8 = Flamework.createSerializer<Serialization.i8>();
export const u16 = Flamework.createSerializer<Serialization.u16>();
export const i16 = Flamework.createSerializer<Serialization.i16>();
export const u32 = Flamework.createSerializer<Serialization.u32>();
export const i32 = Flamework.createSerializer<Serialization.i32>();
export const u64 = Flamework.createSerializer<Serialization.u64>();
export const i64 = Flamework.createSerializer<Serialization.i64>();
export const f32 = Flamework.createSerializer<Serialization.f32>();
export const varint = Flamework.createSerializer<Serialization.varint>();
export const implicitU8 = Flamework.createSerializer<Serialization.Implicit.u8>();
export const implicitI64 = Flamework.createSerializer<Serialization.Implicit.i64>();
export const implicitF32 = Flamework.createSerializer<Serialization.Implicit.f32>();
export const implicitVarint = Flamework.createSerializer<Serialization.Implicit.varint>();
export const ownBrand = Flamework.createSerializer<number & { readonly width: "u16" }>();

/** Strings and buffers with every length prefix. */
export const text = Flamework.createSerializer<string>();
export const text8 = Flamework.createSerializer<Serialization.string8>();
export const text16 = Flamework.createSerializer<Serialization.string16>();
export const text32 = Flamework.createSerializer<Serialization.string32>();
export const implicitText8 = Flamework.createSerializer<Serialization.Implicit.string8>();
export const bytes = Flamework.createSerializer<buffer>();
export const bytes16 = Flamework.createSerializer<Serialization.buffer16>();
export const bytes32 = Flamework.createSerializer<Serialization.buffer32>();
export const template = Flamework.createSerializer<`${string}-id`>();
export const upper = Flamework.createSerializer<Uppercase<string>>();

/** Booleans, literals and nothing. */
export const flag = Flamework.createSerializer<boolean>();
export const one = Flamework.createSerializer<"one">();
export const seven = Flamework.createSerializer<7>();
export const mode = Flamework.createSerializer<"walk" | "run" | "fly">();
export const numbers = Flamework.createSerializer<1 | -1 | 2 | 0 | 10 | -3>();
export const mixedLiterals = Flamework.createSerializer<"number" | "a" | 3 | "" | true>();
export const nothing = Flamework.createSerializer<undefined>();
export const maybe = Flamework.createSerializer<number | undefined>();
export const maybeText = Flamework.createSerializer<string | undefined>();

/** Values that travel whole. */
export const anything = Flamework.createSerializer<unknown>();
export const object = Flamework.createSerializer<object>();
export const part = Flamework.createSerializer<Part>();
export const font = Flamework.createSerializer<Font>();
export const signal = Flamework.createSerializer<RBXScriptSignal>();

class Thing {
	value = 1;
}
export const thing = Flamework.createSerializer<Thing>();
