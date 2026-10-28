import { Flamework } from "@flamework-experimental/core";

// `Promise<T>` is the runtime library's `TS.Promise.is`: a file with no other guard imports no `t`.
export const promise = Flamework.createGuard<Promise<number>>();
