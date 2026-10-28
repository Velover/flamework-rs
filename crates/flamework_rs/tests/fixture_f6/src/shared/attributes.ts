import { BaseComponent, Component, Components } from "@flamework-experimental/components";

interface CounterAttributes {
	count: number;
	label: string;
	flag: boolean;
	note?: string;
	"two words": number;
	target?: BasePart;
}

declare function sideEffect(): number;
declare function pickKey(): "count";

/** Every write to an attribute goes through the setter. */
@Component({ tag: "F6Attributes" })
export class AttributeComponent extends BaseComponent<CounterAttributes, BasePart> {
	public assign(part: BasePart) {
		this.attributes.count = 1;
		this.attributes.label = "text";
		this.attributes.flag = true;
		this.attributes.note = undefined;
		this.attributes.target = part;
		this.attributes["two words"] = 2;
		this.attributes["count"] = 3;
	}

	public compound() {
		this.attributes.count += 1;
		this.attributes.count -= 2;
		this.attributes.count *= 3;
		this.attributes.count /= 4;
		this.attributes.count %= 5;
		this.attributes.count **= 2;
		this.attributes.count |= 1;
		this.attributes.count &= 3;
		this.attributes.count ^= 4;
		this.attributes.count <<= 1;
		this.attributes.count >>= 1;
		this.attributes.count >>>= 1;
		this.attributes.label += "suffix";
		this.attributes.label += this.attributes.count;
		this.attributes["two words"] += 7;
	}

	public logical() {
		this.attributes.flag ||= true;
		this.attributes.flag &&= false;
		this.attributes.note ??= "fallback";
	}

	public updates() {
		this.attributes.count++;
		this.attributes.count--;
		++this.attributes.count;
		--this.attributes.count;
		this.attributes["two words"]++;
	}

	public values() {
		const assigned = (this.attributes.count = 10);
		const added = (this.attributes.count += 5);
		const post = this.attributes.count++;
		const pre = ++this.attributes.count;
		const postDown = this.attributes.count--;
		const preDown = --this.attributes.count;
		const removed = delete this.attributes.note;
		return [assigned, added, post, pre, postDown, preDown, removed];
	}

	public removes() {
		delete this.attributes.note;
		delete this.attributes["note"];
	}

	public effects() {
		this.attributes.count = sideEffect();
		this.attributes.count += sideEffect();
		this.attributes[pickKey()] = 4;
	}

	public nested() {
		const run = () => {
			this.attributes.count += 1;
		};
		run();
		for (let i = 0; i < 3; i++) this.attributes.count++;
		if (this.attributes.flag) this.attributes.label = "flagged";
	}

	public other(other: AttributeComponent, components: Components, part: BasePart) {
		other.attributes.count = 1;
		other.attributes.count++;
		components.getComponent<AttributeComponent>(part)!.attributes.label = "remote";
		delete components.getComponent<AttributeComponent>(part)!.attributes.note;
	}

	/** What the TS transformer does not take for an attribute write: left as written. */
	public notWrites() {
		(this.attributes).count = 1;
		(this.attributes.count) = 2;
		(this.attributes.count)++;
		this["attributes"].count = 3;
		const attributes = this.attributes;
		attributes.count = 4;
	}

	/** Reads are left alone. */
	public reads() {
		const count = this.attributes.count;
		const label = this.attributes["label"];
		return [count, label];
	}
}
