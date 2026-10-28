import { BaseComponent, Component } from "@flamework-experimental/components";

/** An instance type whose child may be another of itself, linked from an attribute. */
interface Branch extends Folder {
	Twig?: Branch;
}

@Component({ tag: "RecursiveLink" })
export class RecursiveLink extends BaseComponent<{ branch: Branch }> {}

/** A recursive tree behind an optional attribute, and a component linked twice. */
@Component({ tag: "F6RecursiveTwice" })
export class RecursiveTwice extends BaseComponent<{ first?: Branch; second: RecursiveLink; third?: RecursiveLink }> {}
