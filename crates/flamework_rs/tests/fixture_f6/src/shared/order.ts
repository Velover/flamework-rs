import { t } from "@rbxts/t";
import { BaseComponent, Component } from "@flamework-experimental/components";

declare function log<T>(tag: string, value: T): T;
declare function maybe(tag: string): number | undefined;

/** Shorthand fields around a written `attributes` the TS moves. */
const tag = "F6Shorthand";
const attributes = { name: t.string };
const defaults = { count: 5 };
@Component({ tag, attributes, defaults })
export class ShorthandComponent extends BaseComponent<{ name: string; count: number }, Part> {}

/** The TS evaluates the fields as it emits them: `tag`, `defaults`, then `attributes`. */
@Component({
	tag: log("m.tag", "F6Moved"),
	attributes: { name: log("m.attr", t.string) },
	defaults: { count: maybe("m.def") ?? 1 },
})
export class MovedComponent extends BaseComponent<{ name: string; count: number }, Part> {}

/** `attributes` already last, but its value needs statements: they run after the fields before it. */
@Component({ tag: log("l.tag", "F6Last"), attributes: { name: (maybe("l.attr") ?? 0) > 0 ? t.string : t.string } })
export class LastComponent extends BaseComponent<{ name: string }, Part> {}

/** A computed name: the fields are still evaluated as the TS emits them. */
@Component({ ["tag"]: log("c.tag", "F6Computed"), attributes: { name: t.string }, defaults: { count: maybe("c.def") ?? 2 } })
export class ComputedComponent extends BaseComponent<{ name: string; count: number }, Part> {}

interface Attributes {
	n: number;
	k: number;
}

/** A receiver and a key that a closure reassigns while the value is computed. */
@Component({ tag: "F6Rebinding" })
export class RebindingComponent extends BaseComponent<Attributes, Part> {
	public rebind(other: RebindingComponent, spare: RebindingComponent) {
		const swap = () => {
			other = spare;
			return log("swap", 1);
		};
		other.attributes.n = maybe("rb") ?? swap();

		let key: "n" | "k" = "n";
		const rekey = () => {
			key = "k";
			return log("rekey", 2);
		};
		this.attributes[key] = maybe("rk") ?? rekey();

		let target = other;
		target.attributes.n = maybe("tc") !== undefined ? ((target = spare), 5) : 6;
	}

	/** Nothing reassigns them, but the value needs statements. */
	public untouched(other: RebindingComponent) {
		other.attributes.n = maybe("u") ?? 3;
		this.attributes.k = maybe("v") ?? 4;
	}
}
