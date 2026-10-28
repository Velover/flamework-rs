//! The names of Flamework's generated locals, as TypeScript's printer gave them (one allocator per file).
//!
//! The TS transformer made every generated local with `factory.createUniqueName(hint, Optimistic)`, and roblox-ts
//! printed the transformed file and parsed it again, so the printer named them: the hint itself when nothing in the
//! file is called that, else `hint_1`, `hint_2`, … (`makeUniqueName`). "Anything in the file" is the parser's
//! `identifiers` table of the *original* file plus every name generated so far in that file.
//!
//! **Order.** The printer names a generated identifier when it first prints it, except that it names the
//! declarations of a block ahead of the block (`generateNames`): those of the whole file's top-level statements
//! first of all, and those of each function body when it starts printing the body. [`crate::serializer::lower`]
//! follows that order within the serializers' code. Across hooks: every serializer of the file is built in
//! `begin_file` and its file-level declarations are named there, ahead of anything another hook names (so an F3
//! guard before the first serializer gets `guard_2` after the serializers' `guard` and `guard_1`, as in TS5). The
//! rest of what a root statement's serializers hoisted is named at the statement's first Flamework hook, as it
//! printed ahead of the statement (so it comes before a guard earlier in the statement), and each serializer's value
//! when its site is built. What still follows hook order rather than print order: names other hooks give at file
//! level (F2's hoisted metadata, F3's dedup locals) against the serializers' nested names. A user macro's generated
//! values are `Arg::Deferred` slots of `compile_call_with` (`crate::macros`), built when the call's lowering reaches
//! them among the written arguments, so the names in written and generated arguments come out in TS5's order
//! (`wrap<T>(Flamework.createGuard<U>())` names the guard first, as TS5's printer did). Exact order everywhere would
//! need the compiler to name plugins' identifiers at render time, in print order (an API taking placeholder
//! identifiers and their hints, named by the renderer as TypeScript's printer named `createUniqueName` identifiers).
//!
//! **The identifiers table** is TS5's parser's `identifiers` of the file, as the compiler lists it
//! (`Cx::file_identifier_texts`: identifiers and private names, JSDoc's included, and the string and numeric
//! literals the parser interned).

use std::collections::HashSet;

use rbxts_plugin::Cx;

/// One file's generated names.
#[derive(Debug, Default, Clone)]
pub struct TsNames {
    file_identifiers: HashSet<&'static str>,
    generated: HashSet<String>,
}

impl TsNames {
    /// The allocator of a file whose identifiers table is `texts` (`Cx::file_identifier_texts`).
    pub fn with_texts(texts: std::rc::Rc<[&'static str]>) -> TsNames {
        TsNames { file_identifiers: texts.iter().copied().collect(), generated: HashSet::new() }
    }

    /// An allocator over a given table (tests).
    #[cfg(test)]
    pub fn with_identifiers(identifiers: &[&'static str]) -> TsNames {
        TsNames { file_identifiers: identifiers.iter().copied().collect(), generated: HashSet::new() }
    }

    fn taken(&self, name: &str) -> bool {
        self.file_identifiers.contains(name) || self.generated.contains(name)
    }

    /// `makeUniqueName(hint, isUniqueName, optimistic = true)`, without asking the compiler (tests).
    #[cfg(test)]
    pub fn unique_plain(&mut self, hint: &str) -> String {
        self.unique_with(hint, |_| true)
    }

    /// `makeUniqueName(hint, isUniqueName, optimistic = true)`. The name is also approved with the compiler
    /// (`is_usable_local_name`), so its temporaries and other plugins avoid it; a name the compiler refuses is
    /// skipped like a taken one.
    pub fn unique(&mut self, cx: &mut Cx<'_>, hint: &str) -> String {
        self.unique_with(hint, |name| cx.is_usable_local_name(name))
    }

    fn unique_with(&mut self, hint: &str, mut approve: impl FnMut(&str) -> bool) -> String {
        if !self.taken(hint) && approve(hint) {
            self.generated.insert(hint.to_owned());
            return hint.to_owned();
        }
        let base = if hint.ends_with('_') { hint.to_owned() } else { format!("{hint}_") };
        let mut i = 1u32;
        loop {
            let name = format!("{base}{i}");
            if !self.taken(&name) && approve(&name) {
                self.generated.insert(name.clone());
                return name;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_make_unique_name() {
        let mut names = TsNames::with_identifiers(&["n", "b", "_"]);
        assert_eq!(names.unique_plain("buf"), "buf");
        assert_eq!(names.unique_plain("buf"), "buf_1");
        assert_eq!(names.unique_plain("n"), "n_1");
        assert_eq!(names.unique_plain("n"), "n_2");
        assert_eq!(names.unique_plain("b"), "b_1");
        assert_eq!(names.unique_plain("_"), "_1");
        assert_eq!(names.unique_plain("_"), "_2");
        assert_eq!(names.unique_plain("x_"), "x_");
        assert_eq!(names.unique_plain("x_"), "x_1");
    }
}
