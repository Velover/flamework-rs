//! F6 builds that need a process variable: obfuscated keys with a pinned seed (`FLAMEWORK_RS_FIXED_SEED`). One test in
//! this binary, so setting it races nothing.

#![cfg(windows)]

mod support;

use flamework_rs::Flamework;
use support::project::{Project, assert_clean, luau};

const SEED: &str = "6f1e2d3c-4b5a-4987-8765-43210fedcba9";

#[test]
fn obfuscated_keys_with_a_pinned_seed() {
    // SAFETY: the only test of this binary; nothing reads the environment concurrently.
    unsafe {
        std::env::set_var("FLAMEWORK_RS_FIXED_SEED", SEED);
    }

    let Some(project) = Project::new("fixture_f6", "obfuscated") else { return };
    project.write("flamework.config.json", "{ \"transformer\": { \"obfuscation\": true } }");
    let first = project.build_with(Flamework::new());
    assert_clean(&first);
    let text = luau(&first, "shared/obfuscation.luau").to_owned();

    let seed = flamework_rs::uuid::Namespace::parse(SEED).unwrap();
    let fire = seed.v5("remotes:fire");
    let call = seed.v5("remotes:call");
    let spaced = seed.v5("remotes:two words");
    for line in [
        format!("local read = remotes[\"{fire}\"]"),
        format!("local element = remotes[\"{fire}\"]"),
        format!("local spaced = remotes[\"{spaced}\"]"),
        // A key that is no identifier: the method is called with an explicit self.
        format!("local called = remotes[\"{call}\"](remotes, 1)"),
        // Through an optional chain the receiver is `Remotes | undefined`, whose marker property is partial: left
        // alone, as the TS leaves it.
        "_chained = _chained.fire".to_owned(),
        // Only the first access's receiver carries the marker.
        format!("local nested = remotes[\"{}\"].inner", seed.v5("remotes:nested")),
        // `obj[k as "k"]` is how obfuscated objects are reached internally: left alone.
        "local cast = remotes[key]".to_owned(),
        format!("remotes[\"{fire}\"] = 1"),
        format!("remotes[\"{fire}\"] += 2"),
        format!("remotes[\"{fire}\"] -= 1"),
        format!("remotes[\"{spaced}\"] = \"x\""),
    ] {
        assert!(text.contains(&line), "missing `{line}`:\n{text}");
    }
    assert_eq!(text.matches(".fire").count(), 1, "{text}");

    // The same seed, the same bytes.
    let second = project.build_with(Flamework::new());
    assert_clean(&second);
    assert_eq!(text, luau(&second, "shared/obfuscation.luau"));
}
