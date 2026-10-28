-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local _core = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out")
local Flamework = _core.Flamework
local Provider = _core.Provider
-- Macro methods of every marker kind; the call sites (`super.m()`, `o.m?.()`, `o?.m()`, `o?.m?.()`) are in the
-- other files.
local Economy
do
	Economy = setmetatable({}, {
		__tostring = function()
			return "Economy"
		end,
	})
	Economy.__index = Economy
	function Economy.new(...)
		local self = setmetatable({}, Economy)
		return self:constructor(...) or self
	end
	function Economy:constructor()
	end
	do
		-- (Flamework) Economy metadata
		Reflect.defineMetadata(Economy, "identifier", "shared/macros@Economy")
		Reflect.defineMetadata(Economy, "flamework:module", script)
		Reflect.defineMetadata(Economy, "flamework:implements", {})
		Reflect.defineMetadata(Economy, "flamework:parameters", {})
		Reflect.defineMetadata(Economy, "flamework:dependencies", {})
	end
	Economy = Provider()(Economy) or Economy
end
local Macros
do
	Macros = setmetatable({}, {
		__tostring = function()
			return "Macros"
		end,
	})
	Macros.__index = Macros
	function Macros.new(...)
		local self = setmetatable({}, Macros)
		return self:constructor(...) or self
	end
	function Macros:constructor()
	end
	function Macros:id(id)
		return id
	end
	function Macros:guard(guard)
		return guard
	end
	function Macros:text(text)
		return text
	end
	function Macros:dependency(info)
		return info
	end
	function Macros:concise(info)
		return info
	end
	function Macros:where(line, character, width, uuid, text)
		return { line, character, width, uuid, text }
	end
	function Macros:obfuscated(name)
		return name
	end
	function Macros:labels(labels)
		return labels
	end
	function Macros:emit(value)
		return value
	end
	function Macros:constant(value)
		return value
	end
	function Macros:path(_path, path)
		return path
	end
	function Macros:glob(_glob, glob)
		return glob
	end
	function Macros:env(_name, value)
		return value
	end
	function Macros:keys(value)
		return value
	end
	function Macros:shuffled(value)
		return value
	end
	function Macros:fieldInfo(meta)
		return meta
	end
	function Macros:tupleGuards(guards)
		return guards
	end
	function Macros:serializer(serializer)
		return serializer
	end
	function Macros:second(first, id)
		return `{first}/{id}`
	end
	function Macros:withOptions(options, id)
		return { options, id }
	end
	function Macros:inlined(text)
		return text
	end
	function Macros:inlinedValue(value)
		return value
	end
	function Macros:implementsOf(object, id)
		return false
	end
end
-- Function macros, for `f?.()` and `ns?.f()`.
-- @metadata macro
local function idOf(id)
	return id
end
-- @metadata macro {@link text intrinsic-inline}
local macros = Macros.new()
local maybe = macros
local holder = {
	macros = macros,
}
local tick = {
	count = 0,
}
local function sideEffect()
	tick.count += 1
	return `side{tick.count}`
end
return {
	idOf = idOf,
	sideEffect = sideEffect,
	Economy = Economy,
	Macros = Macros,
	macros = macros,
	maybe = maybe,
	holder = holder,
	tick = tick,
	Flamework = Flamework,
}
