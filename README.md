# flamework-rs

Flamework's TypeScript transformer (v2, `@flamework-experimental/transformer`) ported to Rust, as a plugin compiled into
[rbxts-rs](#dependencies) (roblox-ts in Rust, on tsrs), plus `rbxtsc-fw`: the `rbxtsc` command line with Flamework
compiled in, the binary a Flamework project builds with.

- `crates/flamework_rs`: the plugin (ids, decorators and metadata, user macros and intrinsics, guards, the serializer,
  components, `flamework.build`, the include JSON) and the byte-exact libraries its output depends on.
- `crates/rbxtsc_fw`: the `rbxtsc-fw` binary. It takes roblox-ts's `build` arguments and returns its exit codes;
  Flamework is on for a project whose tsconfig lists `@flamework-experimental/transformer` in
  `compilerOptions.plugins`.

**Status:** work in progress, provided as is. A later milestone (networking) is still missing; a macro call that needs
it compiles as a plain call, with a warning.

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

The end-to-end tests (`e2e`, `guards_e2e`, `macros`, `macros_env`, `serializer_e2e`) build fixtures with the real
compiler. They need the built Flamework v2 packages (a checkout of its monorepo with `packages/core/out` built:
`FLAMEWORK_RS_MONOREPO`) and the `@rbxts` packages `compiler-types`, `t` and `types` (a `node_modules/@rbxts`
folder: `FLAMEWORK_RS_RBXTS`). Without them they are skipped, saying so. The unit test that checks the embedded JSON
schemas against the transformer's reads the monorepo from `FLAMEWORK_MONOREPO`.

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

## Known limitations

Where flamework-rs's output may differ from Flamework's TS transformer (TypeScript 5) in ways it cannot detect:

- **Property names that start with U+007F (DEL).** rbxts-rs's checker (tsrs) spells its internal names with this
  character, where TypeScript escapes them with `__`, and takes a user name that starts with U+007F followed by any
  character other than `@` or `#` for one of its own: it never lists the property. flamework-rs refuses such a name
  when it is written as a property name of an object, interface or class type (a serializer or a guard of it). Written
  any other way (a const or enum computed key, an intersection, a mapped type, `Record`, `Partial`, `Omit` or a
  `keyof`-derived type, an instance-guard intersection, component attributes (read through the `AttributeGuards`
  mapped type), an instance shape, a `Modding.inspect` or `Many` map, a macro type's fields), the
  property is left out of serializers, guards and macros without an error, and an object left with no fields is
  serialized as a blob. The fix belongs in tsrs.
- **Order that depends on symbol ids.** A union that no written type orders (`Box<A | B>` reaching `value: T`) orders
  its members by a text of each type, and an object's text spells a unique-symbol key with the symbol's id
  (`__@sym@12`). Two keys whose texts agree up to the id (two unique symbols with the same name from different modules,
  or a user key `"__@zeta@1"` against a real `[zeta]`) are ordered by ids that differ between the compilers. TS5's own
  order there depends on its global symbol counter, so it can change with unrelated edits; give such unions a written
  order (a type alias of the union) instead.
