import { Flamework, Modding, Provider, Serialization } from "@flamework-experimental/core";

// Macro methods of every marker kind; the call sites (`super.m()`, `o.m?.()`, `o?.m()`, `o?.m?.()`) are in the
// other files.

@Provider()
export class Economy {}

export interface Shop {
	open(): void;
}

export interface Box {
	value: number;
	tags: string[];
}

export interface Save {
	coins: number;
	readonly userId: number;
	nickname?: string;
}

export class Macros {
	/** @metadata macro */
	id<T>(id?: Modding.Target.Id<T>): string {
		return id!;
	}

	/** @metadata macro */
	guard<T>(guard?: Modding.Target.Guard<T>) {
		return guard!;
	}

	/** @metadata macro */
	text<T>(text?: Modding.Target.Text<T>): string {
		return text!;
	}

	/** @metadata macro */
	dependency<T>(info?: Modding.Target.Dependency<T>): Modding.DependencyInfo {
		return info!;
	}

	/** @metadata macro */
	concise<T>(info?: Modding.Target.DependencyConcise<T>): unknown {
		return info!;
	}

	/** @metadata macro */
	where(
		line?: Modding.Caller.Line,
		character?: Modding.Caller.Character,
		width?: Modding.Caller.Width,
		uuid?: Modding.Caller.Uuid,
		text?: Modding.Caller.Text,
	) {
		return [line, character, width, uuid, text];
	}

	/** @metadata macro */
	obfuscated(name?: Modding.Target.Obfuscate<"playerJoined", "remotes">): string {
		return name!;
	}

	/** @metadata macro */
	labels<T extends readonly unknown[]>(labels?: Modding.Target.Labels<T>): string[] | undefined {
		return labels;
	}

	/** @metadata macro */
	emit<T>(value?: Modding.Emit<T>): T {
		return value!;
	}

	/** @metadata macro */
	constant<T>(value?: Modding.Caller.Constant<Modding.Emit<T>>): T {
		return value!;
	}

	/** @metadata macro */
	path<T extends string>(_path: T, path?: Modding.Intrinsic<"path", [T], string[]>): string[] {
		return path!;
	}

	/** @metadata macro */
	glob<T extends string>(_glob: T, glob?: Modding.Intrinsic<"pathglob", [T], string>): string {
		return glob!;
	}

	/** @metadata macro */
	env<N extends string>(_name: N, value?: Modding.Intrinsic<"env", [N, undefined], string | undefined>) {
		return value;
	}

	/** @metadata macro */
	keys<T>(value?: Modding.Intrinsic<"obfuscate-obj", [T, "remotes"], Record<string, unknown>>) {
		return value!;
	}

	/** @metadata macro */
	shuffled<T>(value?: Modding.Intrinsic<"shuffle-array", [T], unknown[]>) {
		return value!;
	}

	/** @metadata macro */
	fieldInfo<T>(meta?: Modding.Intrinsic<"plugin", ["fieldInfo", T], unknown[]>): unknown[] {
		return meta!;
	}

	/** @metadata macro */
	tupleGuards<T>(guards?: Modding.Intrinsic<"tuple-guards", [T], unknown>) {
		return guards;
	}

	/** @metadata macro */
	serializer<T>(serializer?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>) {
		return serializer!;
	}

	/** Padding: the generated parameter is the second one. */
	/** @metadata macro */
	second<T>(first?: string, id?: Modding.Target.Id<T>): string {
		return `${first}/${id!}`;
	}

	/** A literal argument is required. */
	/** @metadata macro {@link options intrinsic-const} */
	withOptions<T>(options: { name: string }, id?: Modding.Target.Id<T>) {
		return [options, id!] as const;
	}

	/** Inlined: the call is replaced by its generated value. */
	/** @metadata macro {@link text intrinsic-inline} */
	inlined<T>(text?: Modding.Target.Text<T>): string {
		return text!;
	}

	/** Inlined, the value written. */
	/** @metadata macro {@link value intrinsic-inline} */
	inlinedValue<T>(value?: string | Modding.Target.Id<T>): string {
		return value!;
	}

	/** Rewritten: the call goes to `Flamework._implements`. */
	/** @metadata macro {@link Flamework._implements intrinsic-flamework-rewrite} */
	implementsOf<T>(object: unknown, id?: Modding.Target.Id<T>): boolean {
		return false;
	}
}

/** Function macros, for `f?.()` and `ns?.f()`. */
/** @metadata macro */
export function idOf<T>(id?: Modding.Target.Id<T>): string {
	return id!;
}

/** @metadata macro {@link text intrinsic-inline} */
export declare function inlinedText<T>(text?: Modding.Target.Text<T>): string;

export const macros = new Macros();
export const maybe = macros as Macros | undefined;
export const holder = { macros } as { macros?: Macros } | undefined;
export const tick = { count: 0 };
export function sideEffect() {
	tick.count += 1;
	return `side${tick.count}`;
}
export { Flamework };
