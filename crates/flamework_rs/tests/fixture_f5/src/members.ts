import { Events } from "./client";

declare function compute(): number;

// Sends in class members. TypeScript's printer named their locals in source order, where roblox-ts compiles the
// instance properties' initializers first (in the constructor), then the constructor's body, the methods and the
// static properties.
export class Members {
	static first = Events.alpha.fire(compute());
	private events = Events;

	method(value: number) {
		this.events.bravo.fire(tostring(value), compute());
		return this.events.alpha.fire(compute());
	}

	constructor(value: number) {
		this.events.alpha.fire(value * compute());
	}

	arrow = (value: number) => this.events.alpha.fire(value * compute());
	static last = Events.alpha.fire(compute());
}
