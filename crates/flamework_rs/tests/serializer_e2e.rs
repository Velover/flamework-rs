//! The serializer (F4) end to end, through the real compiler on `tests/fixture_f4`: `Flamework.createSerializer<T>()`
//! and a macro of the fixture's own, both user macros whose `serializer` intrinsic reaches the generator
//! (`crate::macros`), every kind of type, one file per family (primitives, datatypes, structures, unions, a file past
//! its budget of locals, a user-declared serializer macro, serializer macros nested in each other's arguments, the
//! mapped types whose field order is read exactly, the properties TS5 leaves out (unique-symbol keys, private names),
//! a guard named after the serializers' file-level locals, and guards earlier in a serializer's root statement, named
//! after what the serializer hoisted).
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
        let got = luau(&report, &name);
        compared += 1;
        if got != want {
            let line = got.lines().zip(want.lines()).position(|(a, b)| a != b).unwrap_or(0);
            failures.push(format!(
                "{name}: first difference at line {}\n  ours: {}\n  ts5:  {}",
                line + 1,
                got.lines().nth(line).unwrap_or("<eof>"),
                want.lines().nth(line).unwrap_or("<eof>")
            ));
        }
    }
    assert!(compared >= 13, "expected outputs missing ({compared})");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
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

/// TSRS-GAP: the shapes whose field order depends on a mapped symbol's link rbxts-rs's checker does not expose yet are
/// refused at their call, naming the type, rather than encoded in an order that may differ from TS5's: properties a
/// spread copied out of a mapped type, and a mapped type with an `as` clause that may name a key otherwise than by the
/// key itself (`K` in a true branch is narrowed; an `infer K` shadows it). The shapes read exactly, `as` clauses that
/// keep or drop each key and a generic interface's mapped bases among them, are in `fixture_f4/src/mapped.ts`, compared
/// with TS5's output.
#[test]
fn unexposed_mapped_links_are_refused() {
    let Some(project) = Project::new(FIXTURE, "gap") else { return };
    std::fs::remove_dir_all(project.root.join("src")).unwrap();
    project.write(
        "src/gap.ts",
        r#"import { Flamework, Serialization } from "@flamework-experimental/core";
type u8 = Serialization.u8;
interface Base {
	zeta: u8;
	alpha: string;
	mid: boolean;
}
declare const picked: Pick<Base, "zeta" | "alpha">;
const spreadPickValue = () => ({ first: 1 as u8, ...picked });
export type SpreadPick = ReturnType<typeof spreadPickValue>;
declare const recorded: Record<"zz" | "aa", u8>;
const spreadRecValue = () => ({ ...recorded, k: true });
export type SpreadRec = ReturnType<typeof spreadRecValue>;
declare const omitted: Omit<Base, "mid">;
const spreadOmitValue = () => ({ ...omitted, extra: 1 as u8 });
export type SpreadOmit = ReturnType<typeof spreadOmitValue>;
declare const chained: Partial<Pick<Base, "zeta" | "alpha">>;
const spreadChainValue = () => ({ ...chained, n: 2 as u8 });
export type SpreadChain = ReturnType<typeof spreadChainValue>;
export type RemapRev = { [K in "a" | "b" as K extends "a" ? "z" : "y"]: u8 };
export type RemapNum = { [K in 1 | 2 as K extends 1 ? "b" : "a"]: string };
export type RemapMulti = { [K in "a" | "b" as K extends "a" ? "z1" | "z2" : "y"]: u8 };
export type RemapTmpl = { [K in "b" | "a" as `x_${K}`]: u8 };
export type TrueBranch = { [K in "c" | "b" | "a" as K extends "a" | "c" ? K : never]: u8 };
export type Shadowed = { [K in "b" | "a" as K extends infer K ? K : never]: u8 };
declare const remapped: RemapRev;
const spreadRemapValue = () => ({ ...remapped, q: 1 as u8 });
export type SpreadRemap = ReturnType<typeof spreadRemapValue>;

export const a = Flamework.createSerializer<SpreadPick>();
export const b = Flamework.createSerializer<SpreadRec>();
export const c = Flamework.createSerializer<SpreadOmit>();
export const d = Flamework.createSerializer<SpreadChain>();
export const e = Flamework.createSerializer<RemapRev>();
export const f = Flamework.createSerializer<RemapNum>();
export const g = Flamework.createSerializer<RemapMulti>();
export const h = Flamework.createSerializer<RemapTmpl>();
export const i = Flamework.createSerializer<SpreadRemap>();
export const j = Flamework.createSerializer<TrueBranch>();
export const k = Flamework.createSerializer<Shadowed>();
"#,
    );
    let report = project.build_with(Flamework::new());
    let errors = errors(&report);
    let refused: Vec<&String> = errors.iter().filter(|e| e.contains("flamework-rs does not support")).collect();
    assert_eq!(refused.len(), 11, "{errors:#?}");
    assert_eq!(errors.len(), 11, "{errors:#?}");
    let hint = "Write the type out as an interface or an object type instead";
    assert!(refused.iter().all(|e| e.contains(hint)), "{errors:#?}");
    let spread = refused.iter().filter(|e| e.contains("as when a spread copies it")).count();
    let remap = refused.iter().filter(|e| e.contains("a mapped type with an `as` clause")).count();
    assert_eq!((spread, remap), (5, 6), "{errors:#?}");
    for name in ["RemapRev", "RemapNum", "RemapMulti", "RemapTmpl", "TrueBranch", "Shadowed"] {
        assert!(refused.iter().any(|e| e.contains(&format!("'{name}'"))), "{name}: {errors:#?}");
    }
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
