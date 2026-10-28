import { Flamework, Serialization } from "@flamework-experimental/core";

type Wallet = { Coins: number } | { Items: string[] };
type Shape = { kind: "circle"; radius: number } | { kind: "rect"; w: number; h: number };
type Mixed = Instance | Vector3 | { kind: "a"; v: number } | { kind: "b"; s: string } | number[] | "lit" | 5;

interface Alpha {
	alpha: number;
}
interface Zed {
	zed: boolean;
}
interface Box<T> {
	value: T;
}

type Id = number | string;
type Choice = Id | Alpha;

interface Crate {
	n: string;
	at: number;
	s?: boolean;
}
interface None {
	readonly __none: "__none";
}
type PatchOrNone = Partial<Crate> | None;
type Ambiguous = Partial<{ a: number }> | Partial<{ b: string }>;

interface Holder {
	pos: { x: number; y: number; z: number };
}
type MapOrHolder = Map<string, { x: number }> | Holder;

export const wallet = Flamework.createSerializer<Wallet>();
export const shape = Flamework.createSerializer<Shape>();
export const mixed = Flamework.createSerializer<Mixed>();
export const sortOf = Flamework.createSerializer<number | string>();
export const sortOfReversed = Flamework.createSerializer<string | number>();
export const narrow = Flamework.createSerializer<Serialization.u16 | number>();
export const narrowOnly = Flamework.createSerializer<Serialization.u8 | string>();
export const choice = Flamework.createSerializer<Choice>();
export const boxes = Flamework.createSerializer<Box<Alpha | Zed>>();
export const unwritten = Flamework.createSerializer<Array<Alpha | Zed>>();
export const withBoolean = Flamework.createSerializer<boolean | string | Vector3>();
export const withEnum = Flamework.createSerializer<Enum.KeyCode | number>();
export const patchOrNone = Flamework.createSerializer<PatchOrNone>();
export const ambiguous = Flamework.createSerializer<Ambiguous>();
export const mapOrHolder = Flamework.createSerializer<MapOrHolder>();
export const optionalUnion = Flamework.createSerializer<Shape | undefined>();
export const nestedUnion = Flamework.createSerializer<Array<Wallet | Shape>>();
export const blobUnion = Flamework.createSerializer<unknown[] | string>();
export const cframes = Flamework.createSerializer<Set<CFrame | string>>();
