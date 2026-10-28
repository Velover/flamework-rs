# flamework-rs

Flamework's TypeScript transformer (v2, `@flamework-experimental/transformer`) ported to Rust, as a plugin compiled into
[rbxts-rs](#dependencies) (roblox-ts in Rust, on tsrs), plus `rbxtsc-fw`: the `rbxtsc` command line with Flamework
compiled in, the binary a Flamework project builds with.

- `crates/flamework_rs`: the plugin (ids, decorators and metadata, user macros and intrinsics, guards, the serializer,
  components, `flamework.build`, the include JSON) and the byte-exact libraries its output depends on.
- `crates/rbxtsc_fw`: the `rbxtsc-fw` binary. It takes roblox-ts's `build` arguments and returns its exit codes;
  Flamework is on for a project whose tsconfig lists `@flamework-experimental/transformer` in
  `compilerOptions.plugins`.

**Status:** work in progress, provided as is. Every part of the TS transformer is ported, networking included; where
the output may still differ, see [Known limitations](#known-limitations).

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
- **Object rests are refused (not an output difference).** TypeScript 5 lists the properties of an object rest
  (`const { a, ...rest } = value`) in the order it lists those of `value`'s type; rbxts-rs's TypeScript 5 property
  order does not rebuild that for a rest yet and keeps TypeScript 7's (by declaration), which differs where `value`'s
  type is a spread (`{ a, ...b }`), inherits a member it redeclares, or has late-bound members. So a serializer of a
  rest whose field order depends on that list is refused, even where the two orders happen to agree: any rest with
  more than one property, except one whose properties a single mapped type (`Pick`, `Omit`, `Record`, ...) made,
  which goes in that type's own order, or whose properties go by name. A type built from such a rest is refused too
  where its own field order follows its list: a spread of the rest, an interface extending it, a homomorphic mapped
  type (`Readonly`, `Partial`, ...) over one of those, wherever they are nested. Write such a type out as an
  interface or an object type. The fix belongs in rbxts-rs.
- **Guard text of an object rest.** A guard of an object rest (or of a type built from one) lists its keys
  (`t.interface({ ... })`) in TypeScript 7's order rather than TypeScript 5's. The guard checks the same keys, so only
  the generated text differs, not what it accepts or anything on the wire. Same cause as above.
- **Order that depends on symbol ids.** A union that no written type orders (`Box<A | B>` reaching `value: T`) orders
  its members by a text of each type, and an object's text spells a unique-symbol key with the symbol's id
  (`__@sym@12`). Two keys whose texts agree up to the id (two unique symbols with the same name from different modules,
  or a user key `"__@zeta@1"` against a real `[zeta]`) are ordered by ids that differ between the compilers. TS5's own
  order there depends on its global symbol counter, so it can change with unrelated edits; give such unions a written
  order (a type alias of the union) instead.

Where it differs from TS5's output in shape only, with the same behaviour:

- **The order of a network's members.** The keys of a mapped type over `keyof T` (`incomingIds`, `outgoingIds`,
  `namespaceIds`, and the decoders built for them) are listed by a key of each, where TS5 lists them in the order its
  checker first made their literal types, which `keyof` does not fix (it shifts with unrelated edits). The arrays hold
  the members' names and remotes are matched by name, so their order carries no meaning: a server and a client built
  by the two compilers work together. The order the codecs are generated in and their names follow it.
- **Packed sends in an optional chain** (`events?.alpha.fire(x)`): the packing goes inside the compiler's nil check,
  where TS5 wraps the call in a function that checks the chain's start, packs, and then evaluates the chain again.
  The behaviour is the same unless an argument changes what the chain starts from (`box.ev?.alpha.fire(f())` where
  `f` assigns `box.ev`): TS5 sends through the new value, flamework-rs through the one read before the arguments.
- **A returned send past its function's budget of locals** (`return events.alpha.fire(x)` in a function that already
  holds many of Flamework's locals): the call and its packing go in a function, where TS5 puts the `return` statement
  in a block of its own with the packing.
- **A send in a `for` loop's incrementor** (`for (let i = 0; i < n; events.alpha.fire(i), i++)`): TS5's packing
  function makes roblox-ts see the loop variable captured and copy it per iteration (`local i = _i` … `_i = i`);
  flamework-rs's packing is not seen as a closure, so the loop has no copies. The function is called where it is made,
  so nothing can see the difference.

Where it differs on purpose:

- **Obfuscated keys through an optional chain** (`maybe?.alpha.fire(x)`, `f()?.ns.leaf.fire(x)` with
  `transformer.obfuscation`): flamework-rs obfuscates the member name as it does outside a chain. TS5 reads the key
  obfuscation marker off the receiver's type as it is, which may be `undefined` there, so it leaves the name as
  written and the send reaches no remote.
- **`middleware` written as a shorthand property, a method or an accessor** (`createServer({ middleware })`): a build
  error, as for any value that is not an object literal. TS5 fails an internal assertion.

What flamework-rs refuses that TS5 builds:

- **Spreading a map, an iterable function or a generator into a packed call's arguments** (`events.list.fire(...map)`):
  arrays, tuples, strings and sets are spread as roblox-ts spreads them; the rest is a build error. Spread the values
  into an array first (`[...map]`).
