import { BaseComponent, Component } from "@flamework-experimental/components";

declare function log<T>(tag: string, value: T): T;
declare function maybe(tag: string): number | undefined;

interface Attributes {
	n: number;
	k: number;
}

/**
 * Receivers and keys a class field initializer reassigns while the value is computed. roblox-ts counts an initializer
 * as the enclosing function's code, so it holds nothing here: the setter reads the reassigned receiver and key.
 */
@Component({ tag: "F6Fields" })
export class FieldsComponent extends BaseComponent<Attributes, Part> {
	public receiver(other: FieldsComponent, spare: FieldsComponent) {
		class Swapper {
			value = ((other = spare), log("field", 1));
		}
		other.attributes.n = maybe("receiver") ?? new Swapper().value;
	}

	public key() {
		let key: "n" | "k" = "n";
		class Rekey {
			value = ((key = "k"), log("rekey", 2));
		}
		this.attributes[key] = maybe("key") ?? new Rekey().value;
	}

	/** A receiver a closure reassigns: held even when the value needs no statements. */
	public closure(other: FieldsComponent, spare: FieldsComponent) {
		const swap = () => {
			other = spare;
			return 3;
		};
		other.attributes.n = swap();
	}
}
