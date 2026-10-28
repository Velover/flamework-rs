import { t } from "@rbxts/t";
import { BaseComponent, Component } from "@flamework-experimental/components";

/** A getter named `attributes`, last and with no links: the TS drops it for the generated guards. */
@Component({
	tag: "F6Getter",
	get attributes() {
		return { name: t.string };
	},
})
export class GetterAttributes extends BaseComponent<{ name: string }, Part> {}

/** A setter named `attributes`: dropped the same way. */
@Component({ tag: "F6Setter", set attributes(value: { [key: string]: t.check<unknown> }) {} })
export class SetterAttributes extends BaseComponent<{ name: string }, Part> {}
