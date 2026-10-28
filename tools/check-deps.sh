#!/usr/bin/env bash
# Fails when the dependency graph holds two copies of a tsrs or rbxts-rs crate.
#
# Cargo does not enforce one copy of a git dependency: when this workspace names tsrs (or rbxts-rs) at another URL or
# rev than rbxts-rs itself does, it builds a second copy, and the build only fails later, with E0308 ("expected
# `tsrs_ast::Node`, found `tsrs_ast::Node`"). Run it after changing a git dependency, a [patch] or Cargo.lock.
#
# usage: bash tools/check-deps.sh   (from anywhere; checks this workspace)
set -euo pipefail
cd "$(dirname "$0")/.."

status=0

# 1. cargo tree -d: every package with more than one copy in the graph, all targets and edge kinds. Each duplicate is
# a root line (no indent) of the inverted trees it prints.
tree="$(cargo tree --workspace --duplicates --target all --edges normal,build,dev)" \
	|| { echo "check-deps: cargo tree failed" >&2; exit 2; }
dups="$(printf '%s\n' "$tree" | grep -E '^(tsrs|rbxts)_[A-Za-z0-9_-]+ v' | sort -u || true)"
if [ -n "$dups" ]; then
	echo "check-deps: duplicate tsrs/rbxts crates in the graph (cargo tree -d):"
	echo "$dups" | sed 's/^/  /'
	status=1
fi

# 2. Cargo.lock: the same name locked twice (a different source or rev counts as a second copy).
lockdups="$(grep -E '^name = "(tsrs|rbxts)_' Cargo.lock | sort | uniq -d || true)"
if [ -n "$lockdups" ]; then
	echo "check-deps: tsrs/rbxts crates locked more than once in Cargo.lock:"
	echo "$lockdups" | sed 's/^/  /'
	status=1
fi

if [ "$status" -eq 0 ]; then
	echo "check-deps: ok ($(grep -cE '^name = "tsrs_' Cargo.lock) tsrs crates, $(grep -cE '^name = "rbxts_' Cargo.lock) rbxts crates, one copy each)"
fi
exit "$status"
