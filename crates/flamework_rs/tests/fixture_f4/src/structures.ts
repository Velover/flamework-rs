import { Flamework, Serialization } from "@flamework-experimental/core";

interface Point {
	x: number;
	y: number;
}

interface Entity {
	id: Serialization.u16;
	name: Serialization.string8;
	tags: string[];
	where: Point;
	mode: "a" | "b" | "c";
	maybe?: number;
	kind: "entity";
	owner: Instance;
}

interface Base {
	base: number;
	shared: string;
}

interface Derived extends Base {
	own: boolean;
	shared: string;
}

interface Keys {
	10: number;
	"two words": string;
	[-1]: boolean;
	plain: number;
}

interface Tree {
	value: number;
	children: Tree[];
}

interface Chain {
	value: Serialization.u8;
	next?: Chain;
}

type Patch<T> = T extends object ? { [K in keyof T]?: Patch<T[K]> } : T;

enum Level {
	Low,
	Mid = 5,
	High = 10,
}

enum Named {
	Alpha = "alpha",
	Beta = "beta",
}

export const point = Flamework.createSerializer<Point>();
export const entity = Flamework.createSerializer<Entity>();
export const derived = Flamework.createSerializer<Derived>();
export const keys = Flamework.createSerializer<Keys>();
export const tree = Flamework.createSerializer<Tree>();
export const chain = Flamework.createSerializer<Chain>();
export const patch = Flamework.createSerializer<Patch<Tree>>();
export const partial = Flamework.createSerializer<Partial<Point>>();
export const record = Flamework.createSerializer<Record<"b" | "a" | "c", number>>();
export const levelRecord = Flamework.createSerializer<Record<Level, string>>();
export const picked = Flamework.createSerializer<Pick<Entity, "name" | "id">>();
export const intersection = Flamework.createSerializer<Point & { z: number }>();
export const level = Flamework.createSerializer<Level>();
export const named = Flamework.createSerializer<Named>();
export const levels = Flamework.createSerializer<Level.Low | Level.High>();

/** Collections. */
export const numbers = Flamework.createSerializer<number[]>();
export const texts = Flamework.createSerializer<string[]>();
export const readonlyTexts = Flamework.createSerializer<ReadonlyArray<string>>();
export const grid = Flamework.createSerializer<number[][]>();
export const set = Flamework.createSerializer<Set<string>>();
export const fixedSet = Flamework.createSerializer<Set<Serialization.u8>>();
export const map = Flamework.createSerializer<Map<string, number>>();
export const fixedMap = Flamework.createSerializer<Map<Serialization.u8, Vector3>>();
export const readonlyMap = Flamework.createSerializer<ReadonlyMap<string, ReadonlySet<number>>>();
export const nested = Flamework.createSerializer<Map<Instance, Array<Set<string>>>>();
export const indexed = Flamework.createSerializer<{ [key: string]: Point }>();
export const markers = Flamework.createSerializer<Array<Array<{ readonly type: "marker" }>>>();

/** Tuples. */
export const pair = Flamework.createSerializer<[number, string]>();
export const optionalTuple = Flamework.createSerializer<[number, string?, ...boolean[]]>();
export const restFirst = Flamework.createSerializer<[...string[], boolean]>();
export const restMiddle = Flamework.createSerializer<[number, ...string[], boolean]>();
export const labelled = Flamework.createSerializer<[at: Vector3, speed?: Serialization.u8]>();

/** Promises unwrap. */
export const promised = Flamework.createSerializer<Promise<Point>>();

/** Versioned. */
export const versioned = Flamework.createSerializer<Entity>({ version: 3 });
export const versionedNumber = Flamework.createSerializer<number>({ version: 0 });
const VERSION = 7;
export const versionedConst = Flamework.createSerializer<[number, string]>({ version: VERSION });
