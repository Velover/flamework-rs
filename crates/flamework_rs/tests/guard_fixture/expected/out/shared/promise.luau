-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Flamework = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Flamework
-- `Promise<T>` is the runtime library's `TS.Promise.is`: a file with no other guard imports no `t`.
local promise = Flamework.createGuard(TS.Promise.is)
return {
	promise = promise,
}
