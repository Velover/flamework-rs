import { Serialization } from "@flamework-experimental/core";
import { Networking } from "@flamework-experimental/networking";

// Members are declared in alphabetical order: TypeScript 5 lists a network's names in the order their literal
// types were made, TypeScript 7 in its own, and declared this way the two agree.

export interface Coord {
	x: number;
	y: number;
}

export interface ServerEvents {
	alpha(value: number): void;
	bravo(name: string, count: Serialization.Implicit.u8): void;
	charlie(): void;
	delta(coord: Coord, tag?: string): void;
	echo(...values: number[]): void;
	foxtrot(first: string, ...rest: number[]): void;
	golf: Networking.Unreliable<(value: number) => void>;
	hotel: Networking.Raw<(value: number) => void>;
	india: Networking.Serialized<(part: Instance, label: string) => void>;
	juliet: { kilo(flag: boolean): void };
	letters(...values: string[]): void;
}

export interface ClientEvents {
	alpha(value: number): void;
	lima(text: string): void;
	mike(count: number, flags: boolean[]): void;
}

export interface ServerFunctions {
	november(a: number): number;
	oscar(name: string): Promise<string>;
	papa(): void;
	quebec(coord: Coord): Coord | undefined;
}

export interface ClientFunctions {
	romeo(value: number): string;
}

export const GlobalEvents = Networking.createEvent<ServerEvents, ClientEvents>();
export const GlobalFunctions = Networking.createFunction<ServerFunctions, ClientFunctions>();
