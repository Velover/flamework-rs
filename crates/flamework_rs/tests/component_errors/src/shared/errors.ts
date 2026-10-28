import { t } from "@rbxts/t";
import { BaseComponent, Component, ComponentConfig } from "@flamework-experimental/components";

@Component({ tag: "Handler" })
export class Handler extends BaseComponent<{}, Part> {}

class NotAnInstance {
	value = 1;
}

/** A child that may be missing. */
@Component({ tag: "OptionalChild" })
export class OptionalChild extends BaseComponent<{}, Model & { Head?: Part }> {}

/** A child below a child that may be missing. */
@Component({ tag: "OptionalGrandchild" })
export class OptionalGrandchild extends BaseComponent<{}, Model & { Body: Folder & { Inner?: Part } }> {}

/** A component below a direct child. */
@Component({ tag: "DeepLink" })
export class DeepLink extends BaseComponent<{}, Model & { Body: Folder & { Inner: Handler } }> {}

interface Branch extends Folder {
	Twig: Branch;
}

/** A child that holds its own tree again. */
@Component({ tag: "EndlessTree" })
export class EndlessTree extends BaseComponent<{}, Model & { Branch: Branch }> {}

/** An attribute no guard can be made for. */
@Component({ tag: "BadAttribute" })
export class BadAttribute extends BaseComponent<{ thing: NotAnInstance }> {}

/** An instance attribute with a child no guard can be made for. */
@Component({ tag: "BadLink" })
export class BadLink extends BaseComponent<{ target: Model & { Child: NotAnInstance } }> {}

/** A config that is not a literal. */
const shared: ComponentConfig = { tag: "Shared" };

@Component(shared)
export class VariableConfig extends BaseComponent<{ amount: number }, Folder> {}

/** A spread in the config. */
const extra = { refreshAttributes: true };

@Component({ ...extra, tag: "Spread", attributes: { count: t.number } })
export class SpreadConfig extends BaseComponent<{ count: number; other: number }> {}

/** Key obfuscation needs the key written down. */
interface Remotes {
	readonly _flamework_key_obfuscation: "remotes";
	fire: number;
	call(): void;
}

declare const remotes: Remotes;
declare const name: "fire";
declare const anyName: string;

export const viaConstant = remotes[name];
export const viaString = remotes[anyName as "fire"];
export const viaTemplate = remotes[`fire`];
export const viaParentheses = remotes[("fire")];

/** A method in a config whose `attributes` the TS moves. */
@Component({
	tag: "Method",
	attributes: { name: t.string },
	predicate(instance) {
		return instance.IsA("Part");
	},
})
export class MethodConfig extends BaseComponent<{ name: string; count: number }, Part> {}

/** A method in a config with no field moved: the TS rebuilds that literal too. */
@Component({
	tag: "Method2",
	predicate(instance) {
		return instance.IsA("Part");
	},
})
export class UnmovedMethodConfig extends BaseComponent<{ name: string }, Part> {}

/** A method in a written `attributes` object, which the TS rebuilds. */
@Component({
	tag: "NestedMethod",
	attributes: {
		name(value: unknown) {
			return value !== undefined;
		},
	},
})
export class NestedMethodConfig extends BaseComponent<{ name: string }, Part> {}

/** An accessor among fields the TS moves. */
@Component({
	attributes: { name: t.string },
	get tag() {
		return "Accessor";
	},
})
export class AccessorConfig extends BaseComponent<{ name: string }, Part> {}

@Component({ tag: "DuplicateTarget" })
export class DuplicateTarget extends BaseComponent<{}, Part> {}

/** A written `links` next to the generated one (TS1117 in the rebuilt literal). */
@Component({ tag: "DuplicateLinks", links: [] })
export class DuplicateLinks extends BaseComponent<{ target: DuplicateTarget }, Part> {}

interface Headed extends Part {
	Head: Part;
}

/** A written `instanceShape` next to the generated one. */
@Component({ tag: "DuplicateShape", instanceShape: { isA: ["Part"] } })
export class DuplicateShape extends BaseComponent<{}, Headed> {}

declare function keyOf<T>(tag: string, value: T): T;

/** A computed attribute name whose literal type is a generated guard's. */
@Component({ tag: "DuplicateGuard", attributes: { [keyOf("k", "name")]: t.string } })
export class DuplicateGuard extends BaseComponent<{ name: string }, Part> {}
