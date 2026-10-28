/** A type whose keys the transformer obfuscates (as networking's remotes are). */
interface Remotes {
	readonly _flamework_key_obfuscation: "remotes";
	fire: number;
	"two words": string;
	call(value: number): number;
	nested: { inner: number };
}

declare const remotes: Remotes;
declare const maybe: Remotes | undefined;
declare const key: "fire";

export const read = remotes.fire;
export const element = remotes["fire"];
export const spaced = remotes["two words"];
export const called = remotes.call(1);
export const chained = maybe?.fire;
export const chainedCall = maybe?.call(2);
export const nested = remotes.nested.inner;
export const cast = remotes[key as "fire"];

export function writes() {
	remotes.fire = 1;
	remotes.fire += 2;
	remotes.fire++;
	--remotes["fire"];
	remotes["two words"] = "x";
}
