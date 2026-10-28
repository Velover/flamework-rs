//! Macro calls through `super.m()`, `super.m?.()`, `o.m?.()`, `o?.m()` and `o?.m?.()` end to end, through the real
//! compiler on `tests/fixture_calls`: macro methods of every marker kind (`src/shared/macros.ts`) called in each shape
//! (`optcall.ts`, `optrecv.ts`, `optboth.ts`, `super.ts`, the same marker list in each), longer chains, function
//! and core macros in chains, callees that are not macros in TS5 (`o["m"]()`, `(o.m)()`, `o.m!()`), and guards in a
//! written argument ahead of a generated serializer (`argguard.ts`: named in TS5's print order), and calls TS5
//! leaves alone because the callee's own symbol is not the macro, whose serializers must not be made (`al.ts`, `ob.ts`,
//! `vleak.ts`: an alias, a property holding the macro, a callee holding a call).
//!
//! `tests/fixture_calls/expected/out` holds the TS5 pipeline's output (roblox-ts master with the published
//! 2.0.0-beta.0 transformer, `fieldInfoPlugin.cjs` as its plugin), each file compared whole.
//!
//! `tests/fixture_chains` holds inlined and rewritten macro calls that end optional chains, which TS5 compiles by
//! replacing the call node (the `crate::macros` docs): `ts5-reference/chaingaps.luau` is TS5's output for the source.

#![cfg(windows)]

mod support;

use support::fixture_macros::flamework_with_fixture_macros;
use support::project::{Project, assert_clean, fixture_dir, luau};

const FIXTURE: &str = "fixture_calls";

fn first_difference(name: &str, got: &str, want: &str) -> String {
    let line = got.lines().zip(want.lines()).position(|(a, b)| a != b).unwrap_or(0);
    format!(
        "{name}: first difference at line {}\n  ours: {}\n  want: {}",
        line + 1,
        got.lines().nth(line).unwrap_or("<eof>"),
        want.lines().nth(line).unwrap_or("<eof>")
    )
}

fn expected_files(dir: &std::path::Path, relative: &str, out: &mut Vec<(String, std::path::PathBuf)>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let name = format!("{relative}{}", entry.file_name().to_string_lossy());
        if entry.file_type().unwrap().is_dir() {
            expected_files(&entry.path(), &format!("{name}/"), out);
        } else {
            out.push((name, entry.path()));
        }
    }
}

/// Every call shape and marker equals TS5's output byte for byte.
#[test]
fn every_shape_matches_ts5() {
    let Some(project) = Project::new(FIXTURE, "shapes") else { return };
    let report = project.build_with(flamework_with_fixture_macros());
    assert_clean(&report);

    let mut files = Vec::new();
    expected_files(&fixture_dir(FIXTURE).join("expected/out"), "", &mut files);
    assert!(files.len() >= 10, "expected outputs missing ({})", files.len());
    let mut failures = Vec::new();
    for (name, path) in &files {
        let want = std::fs::read_to_string(path).unwrap().replace("\r\n", "\n");
        let got = luau(&report, name);
        if got != want {
            failures.push(first_difference(name, got, &want));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Inlined and rewritten macro calls that end an optional chain, compiled as TS5 replaced the call node (41 calls: 28
/// inlined, 13 rewritten): `fixture_chains/ts5-reference/chaingaps.luau` is TS5's output for the same source, compared
/// whole.
#[test]
fn inlined_and_rewritten_calls_in_optional_chains_match_ts5() {
    let Some(project) = Project::new("fixture_chains", "chains") else { return };
    let report = project.build_with(flamework_with_fixture_macros());
    assert_clean(&report);
    let path = fixture_dir("fixture_chains").join("ts5-reference/chaingaps.luau");
    let want = std::fs::read_to_string(path).unwrap().replace("\r\n", "\n");
    let got = luau(&report, "shared/chaingaps.luau");
    assert!(got == want, "{}", first_difference("chaingaps.luau", got, &want));
}
