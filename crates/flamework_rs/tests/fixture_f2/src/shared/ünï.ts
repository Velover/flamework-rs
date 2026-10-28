// A class in a file with a non-ASCII name: its id is a synthesized string with `\u{…}` escapes.

export const label = "ünï";

/** @metadata reflect identifier flamework:parameter_names */
function Named() {
	return (ctor: object) => print(ctor);
}

@Named()
export class Tagged {
	constructor(readonly size: number) {}
}
