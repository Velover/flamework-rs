//! rbxts-rs's W1 harness (`rbxts_transform::build::parity`, DESIGN §14) with Flamework on `tests/fixture`: after
//! every step of a script, a watcher's tree, and an incremental build's in a new session, equal byte for byte a clean
//! build seeded with the plugin state it started from, warnings included. In `full`, `short` and `obfuscated` mode
//! (watch) and `full` and `short` mode (incremental): short ids, the salt, `Modding.Hash` literals and
//! `flamework.build` included.
//!
//! The seed is pinned (`FLAMEWORK_RS_FIXED_SEED`, which pins a new build info's salt too): the initial build and its
//! clean twin each make a new `flamework.build`, and the first build that uses `Modding.Hash` picks its seed, which a
//! clean twin could not pick alike otherwise. Every later choice comes from the state. One test in this binary, so
//! setting the variable races nothing.
//!
//! Also: changed inputs, which the TS transformer refuses, build as clean builds with the new inputs (decision A).
//!
//! Not under obfuscation for incremental builds: those warn that they kept the obfuscated ids, which a clean build
//! does not.

#![cfg(windows)]

mod support;

use std::collections::BTreeMap;
use std::path::Path;

use rbxts_plugin::BuildKind;
use rbxts_plugin::erased::{DynPlugin, erase};
use rbxts_project::{ProjectOptionLayer, STACK_SIZE};
use rbxts_transform::build::parity::{self, NodeModules, ParityOptions};
use rbxts_transform::{BuildSession, CompileOptions, PluginOptions};
use support::project::{Project, errors};

const SEED: &str = "5b0f3c1e-7d2a-4e9b-8c6f-1a2b3c4d5e6f";

const HASH_A: &str = "import { Modding } from \"@flamework-experimental/core\";\n\n/** @metadata macro */\nexport function probeHash(h?: Modding.Target.Hash<\"s2Probe\">) {\n\treturn h;\n}\n\nexport const a = probeHash();\n";
const HASH_B: &str = "import { probeHash } from \"./hashA\";\n\nexport const b = probeHash();\n";

fn big_stack<T: Send>(f: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|scope| {
        std::thread::Builder::new().stack_size(STACK_SIZE).spawn_scoped(scope, f).unwrap().join().unwrap()
    })
}

/// Every file a build writes (`out`, `include`, `flamework.build`), without the incremental state.
fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if entry.file_type().unwrap().is_dir() {
                walk(root, &path, out);
            } else if !entry.file_name().to_string_lossy().ends_with(".rbxts-rs.buildinfo") {
                let relative = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
                out.insert(relative, std::fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, &root.join("out"), &mut out);
    walk(root, &root.join("include"), &mut out);
    if let Ok(bytes) = std::fs::read(root.join("flamework.build")) {
        out.insert("flamework.build".into(), bytes);
    }
    out
}

/// The files of `a` and `b` that differ or are in one only.
fn differences(a: &BTreeMap<String, Vec<u8>>, b: &BTreeMap<String, Vec<u8>>) -> Vec<String> {
    let mut out: Vec<String> = a
        .iter()
        .filter(|(path, bytes)| b.get(*path) != Some(*bytes))
        .map(|(path, _)| format!("{path}: differs or only in the first"))
        .collect();
    out.extend(b.keys().filter(|p| !a.contains_key(*p)).map(|p| format!("{p}: only in the second")));
    out
}

fn compile_options(root: &Path) -> CompileOptions {
    CompileOptions {
        tsconfig: "tsconfig.json".into(),
        cwd: root.to_string_lossy().into_owned(),
        project_options: ProjectOptionLayer {
            optimized_loops: Some(true),
            allow_comment_directives: Some(true),
            ..ProjectOptionLayer::default()
        },
        out_dir_override: None,
        write_partial: false,
        no_write: false,
        checkers: None,
    }
}

/// `Flamework.env` of a variable a `.env` file sets and of one only the process has.
const ENV_USE: &str = "import { Flamework } from \"@flamework-experimental/core\";\n\nexport const value = Flamework.env(\"S2_VALUE\");\nexport const process = Flamework.env(\"S2_PROCESS\");\n";

/// The baseline: short ids, `Modding.Hash` users, inlined variables; incremental unless it is watched.
fn baseline(project: &Project, watched: bool) {
    if !watched {
        let tsconfig = project.read("tsconfig.json");
        project.write("tsconfig.json", &tsconfig.replace("\"incremental\": false", "\"incremental\": true"));
    }
    project.write(".env", "F1_IDMODE=short\nS2_VALUE=one\n");
    project.write("src/shared/hashA.ts", HASH_A);
    project.write("src/shared/hashB.ts", HASH_B);
    project.write("src/shared/envuse.ts", ENV_USE);
}

/// An input change: what it is, the file it edits, and the edit.
type InputChange = (&'static str, &'static str, fn(&Project));

fn input_changes() -> Vec<InputChange> {
    vec![
        ("the id mode", ".env", |p| p.write(".env", "F1_IDMODE=tiny\nS2_VALUE=one\n")),
        ("a code-shaping option", "flamework.config.json", |p| {
            let config = p.read("flamework.config.json");
            p.write("flamework.config.json", &config.replace("\"transformer\": {", "\"transformer\": {\n\t\t\"salt\": \"changed\","));
        }),
        ("an inlined variable of a .env file", ".env", |p| p.write(".env", "F1_IDMODE=tiny\nS2_VALUE=two\n")),
    ]
}

/// Decision A (deviating from TS's `refuseChangedInputs`): an id mode, code-shaping config or inlined variable that
/// changed under an incremental build or a watcher builds, transforming every file, and gives the bytes of a clean
/// build with the new inputs (and the same pinned seed). A changed id mode or code-shaping config starts a new table;
/// a changed variable of a .env file keeps it (every file is transformed, with the same ids). A variable only the
/// process has, changed between incremental builds, transforms again only the file that inlines it.
fn input_changes_build_as_clean_builds(label: &str, watched: bool) {
    let Some(project) = Project::new("fixture", label) else { return };
    baseline(&project, watched);
    let changes = input_changes();
    big_stack(|| {
        let plugins: Vec<Box<dyn DynPlugin>> = vec![erase(flamework_rs::Flamework::new())];
        let options = PluginOptions { plugins: &plugins, ..PluginOptions::default() };
        let clean_with = |applied: &[InputChange], tag: &str| -> BTreeMap<String, Vec<u8>> {
            let clean = Project::new("fixture", &format!("{label}-clean-{tag}")).unwrap();
            baseline(&clean, watched);
            for (_, _, apply) in applied {
                apply(&clean);
            }
            let (report, summary) = BuildSession::new(compile_options(&clean.root), &options, false).build(&[]);
            assert!(errors(&report).is_empty(), "{label} clean {tag}: {:#?}", errors(&report));
            assert_eq!(summary.kind, Some(BuildKind::Clean));
            snapshot(&clean.root)
        };
        let mut session = BuildSession::new(compile_options(&project.root), &options, watched);
        let (report, _) = session.build(&[]);
        assert!(errors(&report).is_empty(), "{label}: {:#?}", errors(&report));
        for (index, (name, file, apply)) in changes.iter().enumerate() {
            apply(&project);
            let (report, summary) = if watched {
                session.build(&[project.root.join(file).to_string_lossy().replace('\\', "/")])
            } else {
                session = BuildSession::new(compile_options(&project.root), &options, false);
                session.build(&[])
            };
            assert!(errors(&report).is_empty(), "{label}, {name}: {:#?}", errors(&report));
            assert!(summary.all, "{label}, {name}: every file is transformed");
            let expected = if watched { BuildKind::WatchRebuild { generation: index as u64 + 1 } } else { BuildKind::Incremental };
            assert_eq!(summary.kind, Some(expected), "{label}, {name}");
            let diff = differences(&snapshot(&project.root), &clean_with(&changes[..=index], &index.to_string()));
            assert_eq!(diff, Vec::<String>::new(), "{label}, {name}: as a clean build with the new inputs");
        }
        if !watched {
            // SAFETY: the only test of this binary, and no build runs now.
            unsafe {
                std::env::set_var("S2_PROCESS", "b");
            }
            session = BuildSession::new(compile_options(&project.root), &options, false);
            let (report, summary) = session.build(&[]);
            assert!(errors(&report).is_empty(), "{label}: {:#?}", errors(&report));
            assert!(!summary.all);
            let transformed: Vec<String> = report.files.iter().map(|f| f.file_name.replace('\\', "/")).collect();
            assert!(transformed.len() == 1 && transformed[0].ends_with("/src/shared/envuse.ts"), "{transformed:?}");
            let diff = differences(&snapshot(&project.root), &clean_with(&changes, "process"));
            assert_eq!(diff, Vec::<String>::new(), "{label}: a process variable, as a clean build");
            // SAFETY: as above.
            unsafe {
                std::env::set_var("S2_PROCESS", "a");
            }
        }
    });
}

/// Unchanged saves, edits, new and deleted classes (one in a file that sorts first), `Modding.Hash` users.
fn script(prepare_env: &str) -> parity::Script {
    let text = serde_json::json!({
        "prepare": [{ "write": { "file": ".env", "text": prepare_env } }],
        "steps": [
            { "name": "an unchanged save", "do": [{ "touch": "src/server/services/alpha.ts" }], "expect": "ok" },
            { "name": "a body edit", "do": [{ "patch": { "file": "src/server/services/alpha.ts", "find": "onInit() {}", "replace": "onInit() {\n\t\tprint(\"edited\");\n\t}" } }], "expect": "ok" },
            { "name": "a new provider", "do": [{ "write": { "file": "src/server/services/gamma.ts", "text": "import { Provider } from \"@flamework-experimental/core\";\n\n@Provider()\nexport class Gamma {}\n" } }], "expect": "ok" },
            { "name": "Modding.Hash users", "do": [{ "write": { "file": "src/shared/hashA.ts", "text": HASH_A } }, { "write": { "file": "src/shared/hashB.ts", "text": HASH_B } }], "expect": "ok" },
            { "name": "one hash user edited", "do": [{ "append": { "file": "src/shared/hashB.ts", "text": "\n// edit\n" } }], "expect": "ok" },
            { "name": "a second new provider, in a file before the others", "do": [{ "write": { "file": "src/server/services/aardvark.ts", "text": "import { Provider } from \"@flamework-experimental/core\";\n\n@Provider()\nexport class Aardvark {}\n" } }], "expect": "ok" },
            { "name": "a provider deleted", "do": [{ "delete": "src/server/services/gamma.ts" }], "expect": "ok" },
            { "name": "the edited file edited again", "do": [{ "patch": { "file": "src/server/services/alpha.ts", "find": "\"edited\"", "replace": "\"again\"" } }], "expect": "ok" },
            { "name": "an unchanged save at the end", "do": [{ "touch": "src/shared/util.ts" }], "expect": "ok" },
            { "name": "a code-shaping option changed", "do": [{ "patch": { "file": "flamework.config.json", "find": "\"transformer\": {", "replace": "\"transformer\": {\n\t\t\"salt\": \"changed\"," } }], "expect": "ok" }
        ]
    });
    parity::parse_script(&text.to_string()).unwrap()
}

/// Runs [`script`] with the harness: watch rebuilds, or incremental builds in new sessions.
fn run_harness(label: &str, env: &str, incremental: bool) {
    let Some(project) = Project::new("fixture", label) else { return };
    // The harness makes the copy incremental by adding `"incremental": true` first in `compilerOptions`; the fixture's
    // own `"incremental": false` would come later and win.
    let tsconfig = project.read("tsconfig.json");
    assert!(tsconfig.contains("\t\t\"incremental\": false,\n"));
    project.write("tsconfig.json", &tsconfig.replace("\t\t\"incremental\": false,\n", ""));
    let scratch = std::env::temp_dir().join(format!("flamework-rs-harness-{label}-{}", std::process::id()));
    let script = script(env);
    let source = project.root.clone();
    let results = big_stack(|| {
        let plugins: Vec<Box<dyn DynPlugin>> = vec![erase(flamework_rs::Flamework::new())];
        let options = ParityOptions {
            source: source.clone(),
            tsconfig: "tsconfig.json".into(),
            copy: ["src", "tsconfig.json", "default.project.json", "package.json", "flamework.config.json", ".env"]
                .map(str::to_owned)
                .to_vec(),
            scratch: scratch.clone(),
            project_options: ProjectOptionLayer {
                optimized_loops: Some(true),
                allow_comment_directives: Some(true),
                ..ProjectOptionLayer::default()
            },
            plugins: &plugins,
            ignore_unknown_plugins: false,
            node_modules: Some(source.join("node_modules")),
            node_modules_mode: NodeModules::Copy,
            incremental,
            keep_clean_copies: false,
            check_clean: true,
            memory: None,
            checkers: None,
        };
        parity::run(&options, &script)
    })
    .unwrap();
    parity::remove_tree(&scratch);
    for (index, result) in results.iter().enumerate() {
        assert!(result.problems.is_empty(), "{label} step {index} `{}`: {:#?}", result.name, result.problems);
    }
    assert_eq!(results.len(), script.steps.len() + 1, "{label}: every step ran");
    let step = |name: &str| results.iter().find(|r| r.name == name).unwrap();
    // Only what changed is transformed: the state file is no setup input.
    assert_eq!(step("an unchanged save").transformed, 0, "{label}");
    assert_eq!(step("a body edit").transformed, 1, "{label}");
    assert_eq!(step("an unchanged save at the end").transformed, 0, "{label}");
    // Every file only for the changed config (a new `flamework.build`, as a clean build makes it).
    let all: Vec<&str> = results[1..].iter().filter(|r| r.all).map(|r| r.name.as_str()).collect();
    assert_eq!(all, ["a code-shaping option changed"], "{label}");
    if incremental {
        assert!(results[1..].iter().all(|r| r.kind == Some(BuildKind::Incremental)), "{label}");
    }
}

#[test]
fn watch_and_incremental_builds_equal_seeded_clean_builds() {
    // SAFETY: the only test of this binary; nothing reads the environment concurrently.
    unsafe {
        std::env::set_var("FLAMEWORK_RS_FIXED_SEED", SEED);
        std::env::set_var("S2_PROCESS", "a");
    }
    input_changes_build_as_clean_builds("s2-inputs-inc", false);
    input_changes_build_as_clean_builds("s2-inputs-watch", true);
    run_harness("s2-watch-full", "", false);
    run_harness("s2-watch-short", "F1_IDMODE=short\n", false);
    run_harness("s2-watch-obfuscated", "F1_OBFUSCATE=true\nF1_IDMODE=obfuscated\n", false);
    run_harness("s2-inc-full", "", true);
    run_harness("s2-inc-short", "F1_IDMODE=short\n", true);
}
