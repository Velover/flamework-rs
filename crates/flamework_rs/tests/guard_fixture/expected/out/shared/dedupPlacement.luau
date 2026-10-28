-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local Flamework = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Flamework
-- Dedup locals go ahead of the TS statement around the call (the TS's `prereqList`), not inside the Luau
-- expression roblox-ts makes of it.
local Point_1 = t.interface({
	x = t.number,
	y = t.number,
})
local lazy = function()
	return Flamework.createGuard(t.interface({
		a = Point_1,
		b = Point_1,
		c = Point_1,
	}))
end
local function conditional(flag)
	local Point_2 = t.interface({
		x = t.number,
		y = t.number,
	})
	local guard = if flag then Flamework.createGuard(t.interface({
		a = Point_2,
		b = Point_2,
		c = Point_2,
	})) else nil
	return guard
end
local function shortCircuit(flag)
	local Point_3 = t.interface({
		x = t.number,
		y = t.number,
	})
	return flag and Flamework.createGuard(t.interface({
		a = Point_3,
		b = Point_3,
		c = Point_3,
	}))
end
return {
	conditional = conditional,
	shortCircuit = shortCircuit,
	lazy = lazy,
}
