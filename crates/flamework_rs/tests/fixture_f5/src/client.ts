import { Coord, GlobalEvents, GlobalFunctions } from "./network";

export const Events = GlobalEvents.createClient({});
export const Functions = GlobalFunctions.createClient({});

declare function compute(): number;
declare function pick(): typeof Events;
declare const handlers: Array<typeof Events>;

// Arguments read where they are (identifiers, literals, parameters) and bound (anything else); a target that is
// more than a plain read is bound first.
export function shapes(value: number, coord: Coord, part: Part) {
	Events.alpha.fire(value);
	Events.alpha.fire(3);
	Events.alpha.fire(compute());
	Events.bravo.fire("name", 7);
	Events.bravo.fire(`n${value}`, value);
	Events.charlie.fire();
	Events.delta.fire(coord);
	Events.delta.fire({ x: value, y: 2 }, "tag");
	Events.echo.fire();
	Events.echo.fire(1, 2, value);
	Events.foxtrot.fire("first", 1, compute());
	Events.golf.fire(value);
	Events.hotel.fire(value);
	Events.india.fire(part, "label");
	Events.juliet.kilo.fire(true);
	Events.alpha(value + 1);
	pick().alpha.fire(value);
	handlers[0].bravo.fire("x", 1);
	const alias = Events;
	alias.alpha.fire(value);
}

// Spread arguments: the list is gathered first, as roblox-ts gathers an array literal with spreads.
export function spreads(values: number[], more: number[]) {
	Events.echo.fire(...values);
	Events.echo.fire(1, ...values);
	Events.echo.fire(...values, 2);
	Events.echo.fire(...values, ...more);
	Events.foxtrot.fire("first", ...values);
}

// Where the call runs: ahead of its statement when that runs it exactly when the call runs, in a function otherwise.
export function positions(flag: boolean, items: number[]) {
	const sent = flag && Events.alpha.fire(compute());
	const picked = items.size() > 0 ? Events.alpha.fire(items[0]) : undefined;
	const list = [items.size(), Events.alpha.fire(items.size())];
	let i = 0;
	while (i < 3 && (Events.alpha.fire(i), true)) i++;
	for (let j = 0; j < 2; j++) Events.alpha.fire(j * 2);
	if (flag) Events.alpha.fire(compute());
	items.forEach((item) => Events.alpha.fire(item * 2));
	const text = `${Events.charlie.fire()}`;
	return [sent, picked, list, text];
}

export function returned(value: number) {
	return Events.alpha.fire(value * 2);
}

// A send inside another send's argument, and inside a callback.
export function nested(value: number) {
	Events.alpha.fire(
		(() => {
			Events.alpha.fire(value + 1);
			return value;
		})(),
	);
	Events.lima.connect((text) => Events.bravo.fire(text, 1));
}

// Invoking the server and answering it.
export async function functions(coord: Coord) {
	const sum = await Functions.november.invoke(1);
	const name = await Functions.oscar.invokeWithTimeout(5, `n${sum}`);
	await Functions.papa();
	const back = await Functions.quebec.invoke({ x: sum, y: name.size() });
	Functions.romeo.setCallback((value) => `${value}`);
	return back ?? coord;
}

// A class method: `this` is a plain read.
export class Sender {
	private events = Events;

	send(value: number) {
		this.events.alpha.fire(value);
		this.events.bravo.fire(tostring(value), value);
	}
}

// At the top of the file.
Events.alpha.fire(1);
Events.charlie.fire();
Events.bravo.fire("top", compute());
