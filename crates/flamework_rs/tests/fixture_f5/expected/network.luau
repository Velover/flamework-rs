-- Compiled with roblox-ts v3.0.0
local TS = _G[script]
local Networking = TS.import(script, TS.getModule(script, "@flamework-experimental", "networking").out).Networking
-- Members are declared in alphabetical order: TypeScript 5 lists a network's names in the order their literal
-- types were made, TypeScript 7 in its own, and declared this way the two agree.
local GlobalEvents = Networking.createEvent("3c59cc37-b38f-571a-8d2f-b39f713af17e")
local GlobalFunctions = Networking.createFunction("c33256c8-0e57-5391-8f74-0aeede316ec8")
return {
	GlobalEvents = GlobalEvents,
	GlobalFunctions = GlobalFunctions,
}
