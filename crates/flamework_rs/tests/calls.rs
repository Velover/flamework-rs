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
//! `tests/chain_errors` holds the inlined and rewritten macro calls inside optional chains, which flamework-rs refuses
//! until rbxts-rs can lower them as TS5 does (the `crate::macros` docs): `expected-diagnostics.txt` is every error,
//! at its call; `ts5-reference/chaingaps.luau` is TS5's output for the same source, for when they are supported.

#![cfg(windows)]

mod support;

use rbxts_plugin::ts::{Diagnostic, P};
use support::fixture_macros::flamework_with_fixture_macros;
use support::project::{Project, assert_clean, fixture_dir, luau};
use tsrs_compiler::diagnosticwriter::flatten_diagnostic_message;
use tsrs_diagnostics::Category;

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

/// `file:line:column` of a diagnostic's start, columns in UTF-16 from 1, as TypeScript prints them.
fn position(d: P<Diagnostic>) -> String {
    let Some(file) = d.file() else { return String::new() };
    let text = file.get().text();
    let pos = (d.pos().unwrap_or(0) as usize).min(text.len());
    let line = text[..pos].matches('\n').count() + 1;
    let line_start = text[..pos].rfind('\n').map_or(0, |i| i + 1);
    let column = text[line_start..pos].encode_utf16().count() + 1;
    let name = file.get().file_name().rsplit('/').next().unwrap_or_default().to_owned();
    format!("{name}:{line}:{column}")
}

/// An inlined or rewritten macro call inside an optional chain is refused, one error at each call, naming the
/// workaround (41 calls: 28 inlined, 13 rewritten). `UPDATE_EXPECTED=1` rewrites the expected file.
#[test]
fn inlined_and_rewritten_calls_in_optional_chains_are_refused() {
    let Some(project) = Project::new("chain_errors", "errors") else { return };
    let report = project.build_with(flamework_with_fixture_macros());
    let mut all = report.diagnostics.clone();
    for file in &report.files {
        all.extend(file.diagnostics.iter().copied());
    }
    let mut lines = Vec::new();
    for d in all {
        let category = if d.category() == Category::Error { "error" } else { "warning" };
        lines.push(format!("{category} {}: {}", position(d), flatten_diagnostic_message(d, "\n")));
    }
    let ours = lines.join("\n") + "\n";
    let expected_path = fixture_dir("chain_errors").join("expected-diagnostics.txt");
    if std::env::var_os("UPDATE_EXPECTED").is_some() {
        std::fs::write(&expected_path, &ours).unwrap();
        return;
    }
    let expected = std::fs::read_to_string(&expected_path).unwrap().replace("\r\n", "\n");
    assert!(ours == expected, "diagnostics differ\n--- ours\n{ours}");

    let refused = |what: &str| lines.iter().filter(|l| l.starts_with("error chaingaps.ts:") && l.contains(what)).count();
    assert_eq!(refused("cannot compile an inlined macro call inside an optional chain"), 28, "{ours}");
    assert_eq!(refused("cannot compile a rewritten macro call inside an optional chain"), 13, "{ours}");
    assert_eq!(lines.len(), 41, "{ours}");
}
