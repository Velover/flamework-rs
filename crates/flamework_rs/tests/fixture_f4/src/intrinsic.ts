import { Flamework, Modding, Serialization } from "@flamework-experimental/core";

interface Point {
	x: number;
	y: Serialization.u16;
}

/**
 * A serializer macro of the project's own: the `serializer` intrinsic outside core.
 *
 * @metadata macro
 */
function serializerOf<T>(meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>) {
	return meta!;
}

/**
 * A versioned one, its options read from the linked parameter.
 *
 * @metadata macro {@link options intrinsic-serializer-options}
 */
function versionedOf<T>(
	options: Serialization.SerializerOptions,
	meta?: Modding.Intrinsic<"serializer", [T], Serialization.Serializer<T>>,
) {
	return meta!;
}

/**
 * Two serializers from one call, in an object the macro emits.
 *
 * @metadata macro
 */
function pairOf<A, B>(
	meta?: Modding.Emit<{
		first: Modding.Intrinsic<"serializer", [A], Serialization.Serializer<A>>;
		second: Modding.Intrinsic<"serializer", [B], Serialization.Serializer<B>>;
	}>,
) {
	return meta!;
}

export const point = serializerOf<Point>();
export const stored = versionedOf<{ name: string; level: Serialization.u8 }>({ version: 2 });
export const both = pairOf<Point, "on" | "off">();
export const core = Flamework.createSerializer<Point>();

export function nested() {
	// A serializer inside a function: its helpers go ahead of the function's statement.
	return serializerOf<Map<string, Point>>();
}
