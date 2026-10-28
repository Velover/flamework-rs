-- Compiled with roblox-ts v3.0.0
-- A type whose keys the transformer obfuscates (as networking's remotes are).
local read = remotes.fire
local element = remotes.fire
local spaced = remotes["two words"]
local called = remotes.call(remotes, 1)
local _chained = maybe
if _chained ~= nil then
	_chained = _chained.fire
end
local chained = _chained
local _chainedCall = maybe
if _chainedCall ~= nil then
	_chainedCall = _chainedCall:call(2)
end
local chainedCall = _chainedCall
local nested = remotes.nested.inner
local cast = remotes[key]
local function writes()
	remotes.fire = 1
	remotes.fire += 2
	remotes.fire += 1
	remotes.fire -= 1
	remotes["two words"] = "x"
end
return {
	writes = writes,
	read = read,
	element = element,
	spaced = spaced,
	called = called,
	chained = chained,
	chainedCall = chainedCall,
	nested = nested,
	cast = cast,
}
