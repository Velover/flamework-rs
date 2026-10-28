import { Flamework, Serialization } from "@flamework-experimental/core";

type u8 = Serialization.u8;
interface Box<T> {
	value: T;
}

/** An unnamed class expression: TS5 names its symbol `__class`, which sorts before `{` in a union member's key. */
export const AnonC = class {
	k: u8 = 0 as u8;
	m: string = "";
};
declare class Named {
	k: u8;
	z: boolean;
}
export type WithObject = Box<InstanceType<typeof AnonC> | { a: string }>;
export type WithClass = Box<InstanceType<typeof AnonC> | Named>;

export const withObject = Flamework.createSerializer<WithObject>({ version: 1 });
export const withClass = Flamework.createSerializer<WithClass>();
