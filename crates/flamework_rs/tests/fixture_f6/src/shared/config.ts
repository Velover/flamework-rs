import { t } from "@rbxts/t";
import { BaseComponent, Component } from "@flamework-experimental/components";

/** No argument at all: the config is made whole. */
@Component()
export class BareComponent extends BaseComponent<{ health: number }, Model> {}

/** An empty object. */
@Component({})
export class EmptyConfigComponent extends BaseComponent<{ health: number }> {}

/** A guard written by hand for one attribute: the generated one for it is left out. */
@Component({
	tag: "F6Custom",
	attributes: {
		health: t.numberConstrained(0, 100),
	},
	defaults: { health: 100 },
})
export class CustomAttributeComponent extends BaseComponent<{ health: number; name: string }, Part> {}

/** Written with a string key and in another order, after the field the shape follows. */
@Component({
	attributes: { "name": t.string },
	tag: "F6Ordered",
})
export class OrderedConfigComponent extends BaseComponent<{ health: number; name: string }, Part> {}

/** The same attributes as its base: nothing to check twice. */
@Component({ tag: "F6Base" })
export class BaseAttributeComponent extends BaseComponent<{ shared: number; own: string }, BasePart> {}

@Component({ tag: "F6Derived" })
export class DerivedAttributeComponent extends BaseAttributeComponent {}

/** A narrower instance than its base, and an attribute of its own. */
abstract class AbstractBase<A extends object = {}> extends BaseComponent<A & { level: number }, BasePart> {}

@Component({ tag: "F6Narrow" })
export class NarrowComponent extends AbstractBase<{ extra: boolean }> {
	declare instance: Part & { Attachment: Attachment };
}

/** No instance type: `Instance` is the base's own, so there is no shape. */
@Component({ tag: "F6Plain" })
export class PlainComponent extends BaseComponent<{ amount?: number; mode: "a" | "b" }> {}

/** An optional child of a child, a class union and a component deeper in the tree are left to the guard. */
@Component({ tag: "F6Guarded" })
export class GuardedComponent extends BaseComponent<{}, Model & { Body: BasePart | Folder; Extra: Folder & { Inner: Part } }> {}

/** Attributes with every kind of value an attribute takes. */
@Component({ tag: "F6Values" })
export class ValuesComponent extends BaseComponent<
	{
		v2: Vector2;
		v3: Vector3;
		color: Color3;
		udim: UDim2;
		range: NumberRange;
		cframe: CFrame;
		text: "on" | "off";
		number: 1 | 2 | 3;
		either?: string | number;
	},
	Model & { Primary: Part; Folder: Folder & { Nested: Model & { Leaf: MeshPart } } }
> {}

/** Written attributes and a link: the link goes first, the written attributes after it. */
@Component({ attributes: { name: t.string }, tag: "F6LinkedCustom", instanceGuard: t.instanceIsA("Model") })
export class LinkedCustomComponent extends BaseComponent<{ name: string; target: Part }, Model & { Handle: Part }> {}

/** Attributes written as anything but an object literal give way to the generated guards. */
const guards = { name: t.string };

@Component({ attributes: guards, tag: "F6HeldGuards" })
export class HeldGuardsComponent extends BaseComponent<{ name: string; count: number }> {}

/** A computed name: the written fields cannot be addressed one by one, so the literal is reordered. */
@Component({ ["tag"]: "F6Computed", attributes: { name: t.string }, refreshAttributes: true })
export class ComputedNameComponent extends BaseComponent<{ name: string; count: number }, Part> {}
