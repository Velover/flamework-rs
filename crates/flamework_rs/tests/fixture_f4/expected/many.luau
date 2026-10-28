-- Compiled with roblox-ts v3.0.0
local TS = _G[script]
local Modding = TS.import(script, TS.getModule(script, "@flamework-experimental", "core").out).Modding
-- `Modding.Many<(A | B)[]>` lists a union's values as the call writes it, which is TS5's order for new types.
local aliased = Modding.inspect({ "zeta", "alpha", "mid" })
local written = Modding.inspect({ "zeta", "alpha" })
local objects = Modding.inspect({ {
	z = "x",
}, {
	b = "z",
}, {
	a = "u",
} })
local symbolKeyed = Modding.inspect({ {
	b = "z",
}, {
	a = "y",
} })
local tagged = Modding.inspect({ {
	k = "z",
}, {
	k = "a",
} })
return {
	aliased = aliased,
	written = written,
	objects = objects,
	symbolKeyed = symbolKeyed,
	tagged = tagged,
}
