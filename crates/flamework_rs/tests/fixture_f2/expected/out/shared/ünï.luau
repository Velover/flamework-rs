-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
-- A class in a file with a non-ASCII name: its id is a synthesized string with `\u{…}` escapes.
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local label = "ünï"
-- @metadata reflect identifier flamework:parameter_names
local function Named()
	return function(ctor)
		return print(ctor)
	end
end
local Tagged
do
	Tagged = setmetatable({}, {
		__tostring = function()
			return "Tagged"
		end,
	})
	Tagged.__index = Tagged
	function Tagged.new(...)
		local self = setmetatable({}, Tagged)
		return self:constructor(...) or self
	end
	function Tagged:constructor(size)
		self.size = size
	end
	do
		-- (Flamework) Tagged metadata
		Reflect.defineMetadata(Tagged, "identifier", "shared/\u{00FC}n\u{00EF}@Tagged")
		Reflect.defineMetadata(Tagged, "flamework:module", script)
		Reflect.defineMetadata(Tagged, "flamework:parameter_names", { "size" })
	end
	Tagged = Named()(Tagged) or Tagged
end
return {
	label = label,
	Tagged = Tagged,
}
