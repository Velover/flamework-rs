//! The serializer (F4) end to end, through the real compiler on `tests/fixture_f4`: `Flamework.createSerializer<T>()`
//! and a macro of the fixture's own, both user macros whose `serializer` intrinsic reaches the generator
//! (`crate::macros`), every kind of type, one file per family (primitives, datatypes, structures, unions, a file past
//! its budget of locals, a user-declared serializer macro, serializer macros nested in each other's arguments, the
//! mapped types, the types that copy or inherit a mapped type's properties (spreads, object rests, a generic
//! interface's instantiated bases) and `as` clauses that rename keys, the properties TS5 leaves out (unique-symbol
//! keys, private names), a guard named after the serializers' file-level locals, and guards earlier in a
//! serializer's root statement, named after what the serializer hoisted).
//!
//! `tests/fixture_f4/expected` holds the TS5 pipeline's output for every file (roblox-ts master with the published
//! 2.0.0-beta.0 transformer), each compared whole. `unions.luau` has union members told apart by F3's guards, which
//! print with the serializer's names (`crate::serializer::ir::Guard`).
//!
//! The project is copied to a temporary directory with a `node_modules` as an install has it (`support::project`);
//! without the monorepo and the `@rbxts` packages the tests are skipped, saying so.

#![cfg(windows)]

mod support;

use flamework_rs::Flamework;
use support::project::{Project, assert_clean, errors, fixture_dir, luau};

const FIXTURE: &str = "fixture_f4";

/// Every file builds, and the files in `expected` equal TS5's byte for byte.
#[test]
fn every_kind_builds_and_matches_ts5() {
    let Some(project) = Project::new(FIXTURE, "kinds") else { return };
    let report = project.build_with(Flamework::new());
    assert_clean(&report);

    let expected = fixture_dir(FIXTURE).join("expected");
    let mut failures = Vec::new();
    let mut compared = 0;
    for entry in std::fs::read_dir(&expected).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        let want = std::fs::read_to_string(entry.path()).unwrap().replace("\r\n", "\n");
        compared += 1;
        failures.extend(first_difference(&name, luau(&report, &name), &want));
    }
    assert!(compared >= 17, "expected outputs missing ({compared})");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Where `got` first differs from TS5's `want`, described.
fn first_difference(name: &str, got: &str, want: &str) -> Option<String> {
    if got == want {
        return None;
    }
    let line = got.lines().zip(want.lines()).position(|(a, b)| a != b).unwrap_or(0);
    Some(format!(
        "{name}: first difference at line {}\n  ours: {}\n  ts5:  {}",
        line + 1,
        got.lines().nth(line).unwrap_or("<eof>"),
        want.lines().nth(line).unwrap_or("<eof>")
    ))
}

/// A serializer's errors carry the TS transformer's text, at the macro call: a version that is not known when the
/// project builds, and a type that cannot be serialized.
#[test]
fn errors_name_the_call() {
    let Some(project) = Project::new(FIXTURE, "errors") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/errors.ts",
        "import { Flamework } from \"@flamework-experimental/core\";\n\
         declare const v: number;\n\
         export const a = Flamework.createSerializer<{ f: () => void }>();\n\
         export const b = Flamework.createSerializer<number>({ version: v });\n\
         export const c = Flamework.createSerializer<number>({ version: 256 });\n",
    );
    let report = project.build_with(Flamework::new());
    let errors = errors(&report);
    let has = |text: &str| errors.iter().any(|e| e.contains(text));
    assert!(has("Flamework cannot serialize this type"), "{errors:#?}");
    assert!(has("so it has to be known when the project builds, and the type of this one is 'number'"), "{errors:#?}");
    assert!(has("A serializer's version is written as one byte: a whole number from 0 to 255, not 256."), "{errors:#?}");
}

/// `Modding.Many<(A | B)[]>` (here `Modding.inspect`) lists a union's values as the call or its alias writes them,
/// which is TS5's creation order for new types (`fixture_f4/src/many.ts` has TS5's bytes), else by a key of each
/// member (DESIGN §10.4): `keyof` writes no order, and TS5's there follows what its checker met first.
#[test]
fn many_lists_a_union_in_written_order() {
    let Some(project) = Project::new(FIXTURE, "many") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/many.ts",
        "import { Modding } from \"@flamework-experimental/core\";\n\
         type Choice = \"zeta\" | \"alpha\" | \"mid\";\n\
         export const written = Modding.inspect<Choice[]>();\n\
         export const inline = Modding.inspect<(\"zeta\" | \"alpha\")[]>();\n\
         export const keyed = Modding.inspect<(keyof { zeta: 1; alpha: 2 })[]>();\n",
    );
    let report = project.build_with(Flamework::new());
    assert_clean(&report);
    let out = luau(&report, "many.luau");
    assert!(out.contains("local written = Modding.inspect({ \"zeta\", \"alpha\", \"mid\" })"), "{out}");
    assert!(out.contains("local inline = Modding.inspect({ \"zeta\", \"alpha\" })"), "{out}");
    assert!(out.contains("local keyed = Modding.inspect({ \"alpha\", \"zeta\" })"), "{out}");
}

/// The field orders that follow a mapped property's links (`links.mappedType`, `links.keyType`), built on their own:
/// properties a spread or an object rest copied out of a mapped type and a generic interface's instantiated mapped
/// bases (`fixture_f4/src/spreads.ts`), and `as` clauses that rename keys, a true branch's narrowed `K`, an `infer K`
/// shadowing it and several keys naming one field among them (`remaps.ts`). Both equal TS5's bytes.
#[test]
fn mapped_links_match_ts5() {
    let Some(project) = Project::new(FIXTURE, "links") else { return };
    for entry in std::fs::read_dir(project.root.join("src")).unwrap() {
        let path = entry.unwrap().path();
        if !path.ends_with("spreads.ts") && !path.ends_with("remaps.ts") {
            std::fs::remove_file(path).unwrap();
        }
    }
    let report = project.build_with(Flamework::new());
    assert_clean(&report);
    let expected = fixture_dir(FIXTURE).join("expected");
    let failures: Vec<String> = ["spreads.luau", "remaps.luau"]
        .into_iter()
        .filter_map(|name| {
            let want = std::fs::read_to_string(expected.join(name)).unwrap().replace("\r\n", "\n");
            first_difference(name, luau(&report, name), &want)
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// RBXTS-GAP: an object rest lists its properties in the order of the type it was taken from, which rbxts-rs's
/// TypeScript 5 property order does not rebuild for a rest yet: a rest of two runs or more (properties of different
/// origins, or several that no one mapped type made) is refused, naming the type, mapped or not, and so is a type of
/// several runs built from one (a spread of it, an interface extending it, a homomorphic mapped type over that,
/// either nested in a container). A rest of one run (one property, or properties a single mapped type made) has an
/// order that does not depend on the rest's list, and builds, as does a spread of it (`fixture_f4/src/spreads.ts`
/// has TS5's bytes for both kinds of rest).
#[test]
fn a_multi_run_object_rest_is_refused() {
    let Some(project) = Project::new(FIXTURE, "rest") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/rest.ts",
        r#"import { Flamework, Serialization } from "@flamework-experimental/core";
type u8 = Serialization.u8;
interface Base {
	zeta: u8;
	alpha: string;
	mid: boolean;
}
// A mapped type's properties and a written one.
declare const omitted: Omit<Base, "mid">;
const spreadValue = () => ({ ...omitted, extra: 1 as u8 });
declare const spread: ReturnType<typeof spreadValue>;
const mixedValue = () => {
	const { alpha: _alpha, ...rest } = spread;
	return rest;
};
// No mapped type: a spread whose TS5 list (right operand first) differs from declaration order.
const lateValue = () => ({ a: 1 as u8, ...late, c: 3 as u8 });
declare const late: Late;
interface Late {
	b: u8;
}
declare const lated: ReturnType<typeof lateValue>;
const plainValue = () => {
	const { c: _c, ...rest } = lated;
	return rest;
};
// No mapped type: an interface redeclaring a member of its base.
interface A {
	x: u8;
	y: u8;
}
interface B extends A {
	y: u8;
	z: u8;
}
declare const b: B;
const inheritedValue = () => {
	const { z: _z, ...rest } = b;
	return rest;
};
// One run each: properties one mapped type made, and a single property.
declare const picked: Pick<Base, "zeta" | "alpha" | "mid">;
const wholeValue = () => {
	const { mid: _mid, ...rest } = picked;
	return rest;
};
const singleValue = () => {
	const { x: _x, y: _y, ...rest } = b;
	return rest;
};
// Types built from a rest of several runs: TS5 lists VRI's as own, alpha, zeta, mid, TypeScript 7 by declaration.
interface VRI extends Base {
	own: u8;
	alpha: string;
}
declare const vri: VRI;
const restValue = () => {
	const { mid: _m, ...rest } = vri;
	return rest;
};
type Rest = ReturnType<typeof restValue>;
declare const rest: Rest;
const h1Value = () => ({ ...rest, k: true });
type H1 = ReturnType<typeof h1Value>;
interface H2 extends Rest {
	k: boolean;
}
type H3 = Readonly<H1>;
type H4 = { list: H1[] };
declare const h1: H1;
const h5Value = () => ({ q: 1 as u8, ...h1 });
const vr25Value = () => {
	const { mid: _m, ...rest } = vri;
	return { ...rest, k: true };
};
// Built from a rest of one run: exact.
declare const whole: ReturnType<typeof wholeValue>;
const fromWholeValue = () => ({ k: true, ...whole });
export const mixed = Flamework.createSerializer<ReturnType<typeof mixedValue>>();
export const plain = Flamework.createSerializer<ReturnType<typeof plainValue>>();
export const inherited = Flamework.createSerializer<{ inner: ReturnType<typeof inheritedValue> }>();
export const wholeS = Flamework.createSerializer<ReturnType<typeof wholeValue>>();
export const single = Flamework.createSerializer<ReturnType<typeof singleValue>>();
export const fromWhole = Flamework.createSerializer<ReturnType<typeof fromWholeValue>>();
export const h1s = Flamework.createSerializer<H1>();
export const h2s = Flamework.createSerializer<H2>();
export const h3s = Flamework.createSerializer<H3>();
export const h4s = Flamework.createSerializer<H4>();
export const h5s = Flamework.createSerializer<ReturnType<typeof h5Value>>();
export const vr25 = Flamework.createSerializer<ReturnType<typeof vr25Value>>();
"#,
    );
    let report = project.build_with(Flamework::new());
    let errors = errors(&report);
    assert_eq!(errors.len(), 9, "{errors:#?}");
    for text in ["{ zeta: u8; extra: u8; }", "{ a: u8; b: u8; }", "{ x: u8; y: u8; }"] {
        let refused = format!("flamework-rs does not support '{text}' yet: it is an object rest");
        assert!(errors.iter().any(|e| e.contains(&refused)), "{text}: {errors:#?}");
    }
    let built = errors.iter().filter(|e| e.contains("yet: it is built from an object rest")).count();
    assert_eq!(built, 6, "H1 to H5 and VR25: {errors:#?}");
}

/// TSRS-GAP: a declared property whose name starts with U+007F (`"\u007fab"`) is one rbxts-rs's checker takes for a
/// reserved name and does not list, where TS5 keeps it: a serializer and a guard of such a type are refused, naming
/// the property, rather than built without it.
#[test]
fn a_property_the_checker_hides_is_refused() {
    let Some(project) = Project::new(FIXTURE, "hidden-name") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/hidden.ts",
        "import { Flamework, Serialization } from \"@flamework-experimental/core\";\n\
         type u8 = Serialization.u8;\n\
         export interface D1 {\n\t\"\\u007fab\": u8;\n\tb: u8;\n}\n\
         export interface D2 extends D1 {\n\tc: u8;\n}\n\
         export const s1 = Flamework.createSerializer<D1>();\n\
         export const s2 = Flamework.createSerializer<{ inner: D2 }>();\n\
         export const g1 = Flamework.createGuard<D1>();\n",
    );
    let report = project.build_with(Flamework::new());
    let errors = errors(&report);
    let refused = errors.iter().filter(|e| e.contains("does not support the property '\\u007fab' yet")).count();
    assert_eq!((refused, errors.len()), (3, 3), "{errors:#?}");
}

/// `checkGlobalsIn` on a union member's guard (F3's): a module-level declaration hiding a global the guard names
/// (`Enum` in `t.enum(Enum.Material)`) is refused, as the TS transformer refuses it.
#[test]
fn a_guard_s_hidden_global_is_refused() {
    let Some(project) = Project::new(FIXTURE, "hidden") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/hidden.ts",
        "import { Flamework } from \"@flamework-experimental/core\";\n\
         const Enum = 1;\n\
         export const used = Enum;\n\
         export const s = Flamework.createSerializer<Map<string, boolean> | Array<{ m: Enum.Material }>>();\n",
    );
    let report = project.build_with(Flamework::new());
    let errors = errors(&report);
    assert!(
        errors.iter().any(|e| e.contains("uses the global 'Enum', which the declaration of 'Enum' on line 2 hides")),
        "{errors:#?}"
    );
}
