import { Networking } from "@flamework-experimental/networking";
import { GlobalEvents, GlobalFunctions } from "./network";

function record<I extends unknown[]>(label: string): Networking.EventMiddleware<I> {
	return (processNext) => (player, ...args) => {
		print(label);
		return processNext(player, ...args);
	};
}

// The middleware's names stay as written without obfuscation.
export const Events = GlobalEvents.createServer({
	middleware: {
		alpha: [record<[number]>("alpha")],
		juliet: { kilo: [record<[boolean]>("kilo")] },
	},
});

export const Functions = GlobalFunctions.createServer({});

// Sends from the server: to one player, to all but one, to everyone, and the member called itself.
export function sendAll(player: Player, others: Player[], text: string) {
	Events.lima.fire(player, text);
	Events.lima.except(player, "skipped");
	Events.lima.broadcast(`${text}!`);
	Events.lima(player, text.upper());
	Events.mike.fire(others, others.size(), [true, false]);
	Events.alpha.broadcast(1);
}

// Callbacks: an arrow, an identifier, a function whose result carries nothing, and a Promise.
export function callbacks() {
	Functions.november.setCallback((player, a) => a + 1);
	const answer = (player: Player, name: string) => Promise.resolve(`hi ${name}`);
	Functions.oscar.setCallback(answer);
	Functions.papa.setCallback(() => {});
	Functions.quebec.setCallback((player, coord) => (coord.x > 0 ? coord : undefined));
}

// Invoking the client, with and without a timeout.
export async function invokeClient(player: Player) {
	const text = await Functions.romeo.invoke(player, 5);
	const again = await Functions.romeo.invokeWithTimeout(player, 2, text.size());
	return Functions.romeo(player, again.size());
}
