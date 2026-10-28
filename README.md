# flamework-rs

Flamework's TypeScript transformer (v2, `@flamework-experimental/transformer`) ported to Rust, as a plugin compiled into
[rbxts-rs](#dependencies) (roblox-ts in Rust, on tsrs), plus `rbxtsc-fw`: the `rbxtsc` command line with Flamework
compiled in, the binary a Flamework project builds with.

- `crates/flamework_rs`: the plugin (ids, decorators and metadata, user macros and intrinsics, guards, components,
  `flamework.build`, the include JSON) and the byte-exact libraries its output depends on.
- `crates/rbxtsc_fw`: the `rbxtsc-fw` binary. It takes roblox-ts's `build` arguments and returns its exit codes;
  Flamework is on for a project whose tsconfig lists `@flamework-experimental/transformer` in
  `compilerOptions.plugins`.

**Status:** work in progress, provided as is. Later milestones (the serializer, networking) are still missing; a macro
call that needs one compiles as a plain call, with a warning.

**Licence:** MIT (`LICENSE`). Parts are ports of Flamework, hashids, uuid, glob, minimatch and brace-expansion; their
notices are in `THIRD-PARTY-NOTICES.md`.

## Dependencies

rbxts-rs (the compiler and its plugin API, `rbxts_plugin`) and tsrs are git dependencies pinned by rev in the root
`Cargo.toml`. The tsrs URL and rev must be exactly the ones rbxts-rs's own `Cargo.toml` names at the pinned rbxts-rs
rev: Cargo does not enforce one copy of a git dependency, so a mismatch builds a second tsrs and fails only later, with
E0308 on a tsrs type. `bash tools/check-deps.sh` fails when any `tsrs_*` or `rbxts_*` crate is in the graph twice; run
it after changing a dependency, a `[patch]` or `Cargo.lock`. `Cargo.lock` started as a copy of rbxts-rs's.

## Build and test

The toolchain comes from `rust-toolchain.toml`. Dev builds are optimised (a debug checker is unusably slow).

```sh
cargo build --locked
cargo test --locked
bash tools/check-deps.sh
```

The end-to-end tests (`e2e`, `guards_e2e`, `macros`, `macros_env`) build fixtures with the real compiler. They need
the built Flamework v2 packages (a checkout of its monorepo with `packages/core/out` built: `FLAMEWORK_RS_MONOREPO`)
and the `@rbxts` packages `compiler-types`, `t` and `types` (a `node_modules/@rbxts` folder: `FLAMEWORK_RS_RBXTS`).
Without them they are skipped, saying so. The unit test that checks the embedded JSON schemas against the
transformer's reads the monorepo from `FLAMEWORK_MONOREPO`.

## Developing against a local rbxts-rs

To build against a local rbxts-rs checkout instead of the pinned rev, override the git source with path dependencies
in an uncommitted `.cargo/config.toml` (git-ignored), one line per rbxts-rs crate in `Cargo.lock`:

```toml
[patch."file:///E:/Projects/TS/Flamework/rbxts-rs"]
rbxts_cli = { path = "<rbxts-rs>/crates/rbxts_cli" }
rbxts_luau = { path = "<rbxts-rs>/crates/rbxts_luau" }
rbxts_luau_check = { path = "<rbxts-rs>/crates/rbxts_luau_check" }
rbxts_plugin = { path = "<rbxts-rs>/crates/rbxts_plugin" }
rbxts_project = { path = "<rbxts-rs>/crates/rbxts_project" }
rbxts_transform = { path = "<rbxts-rs>/crates/rbxts_transform" }
```

While it is on, Cargo rewrites `Cargo.lock` for the path sources, so `--locked` fails: build without it, and restore
`Cargo.lock` (`git checkout Cargo.lock`) when you remove the patch. The local checkout must name the same tsrs as this
workspace (`tools/check-deps.sh`). When the change lands in rbxts-rs, move the rev in `Cargo.toml` to it and update the
lock with `cargo update -p rbxts_plugin` (which moves every crate of that source).

## Parity

`tools/parity.cjs` builds a whole project with `rbxtsc-fw` and compares the output, file by file and byte for byte,
with a reference build of the same project (TypeScript 5 roblox-ts with Flamework's TS transformer):

```sh
node tools/parity.cjs <rbxtsc-fw exe> <project dir> <reference out dir> <work dir> [--report FILE] [--diff N] [--save-out DIR]
```

It copies the project into the work dir (without its `out`, `include` and build infos; links are kept as junctions),
builds it, and prints `build exit N` and `identical A, differing B, only ours C, only ref D` (`tools/compare.cjs`, also
usable on its own). The options are described at the top of `tools/parity.cjs`.
