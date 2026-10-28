-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local ns = TS.import(script, game:GetService("ReplicatedStorage"), "TS", "macros")
local _macros = TS.import(script, game:GetService("ReplicatedStorage"), "TS", "macros")
local Flamework = _macros.Flamework
local holder = _macros.holder
local idOf = _macros.idOf
local macros = _macros.macros
local maybe = _macros.maybe
-- Function macros: `f?.()`, `ns?.f()`, `ns.f?.()`.
local _fnOptional = idOf
if _fnOptional ~= nil then
	_fnOptional = _fnOptional("shared/macros@Economy")
end
local fnOptional = _fnOptional
local _nsReceiver = ns
if _nsReceiver ~= nil then
	_nsReceiver = _nsReceiver.idOf("shared/macros@Economy")
end
local nsReceiver = _nsReceiver
local _nsCall = ns.idOf
if _nsCall ~= nil then
	_nsCall = _nsCall("shared/macros@Economy")
end
local nsCall = _nsCall
-- Core's macros through optional chains (generated, not inlined or rewritten).
local _coreGuard = Flamework
if _coreGuard ~= nil then
	_coreGuard = _coreGuard.createGuard(t.interface({
		value = t.number,
		tags = t.array(t.string),
	}))
end
local coreGuard = _coreGuard
local _coreGuardCall = Flamework.createGuard
if _coreGuardCall ~= nil then
	_coreGuardCall = _coreGuardCall(t.interface({
		value = t.number,
		tags = t.array(t.string),
	}))
end
local coreGuardCall = _coreGuardCall
-- Longer chains: the macro call after more links, links after the macro call, and nested optional chains.
local _deep = holder
if _deep ~= nil then
	_deep = _deep.macros
	if _deep ~= nil then
		_deep = _deep:id("shared/macros@Economy")
	end
end
local deep = _deep
local _deepBoth = holder
if _deepBoth ~= nil then
	local _self = _deepBoth.macros
	_deepBoth = _self
	if _deepBoth ~= nil then
		_deepBoth = _deepBoth.id
		if _deepBoth ~= nil then
			_deepBoth = _deepBoth(_self, "shared/macros@Economy")
		end
	end
end
local deepBoth = _deepBoth
local _result = holder
if _result ~= nil then
	_result = _result.macros
end
local deepPlain = _result:second(macros:text("Box"), "shared/macros@Economy")
local _after = maybe
if _after ~= nil then
	_after = #_after:id("shared/macros@Economy")
end
local after = _after
local _result_1 = maybe
if _result_1 ~= nil then
	_result_1 = _result_1:id("shared/macros@Economy")
end
local _condition = _result_1
if _condition == nil then
	_condition = "none"
end
local coalesced = _condition
local _inlinedFirst = "Economy"
if _inlinedFirst ~= nil then
	_inlinedFirst = #_inlinedFirst
end
local inlinedFirst = _inlinedFirst
local _nestedArgument = maybe
if _nestedArgument ~= nil then
	local _fn = _nestedArgument.second
	local _result_2 = maybe
	if _result_2 ~= nil then
		_result_2 = _result_2:text("Box")
	end
	_nestedArgument = _fn(_nestedArgument, _result_2, "shared/macros@Economy")
end
local nestedArgument = _nestedArgument
-- Callees TypeScript's `getSymbolAtLocation` has no symbol for: not macros in TS5, plain calls.
local _elementCall = maybe
if _elementCall ~= nil then
	_elementCall = _elementCall.id(_elementCall)
end
local elementCall = _elementCall
local elementPlain = macros.id(macros)
local _self = maybe
local _elementBoth = _self
if _elementBoth ~= nil then
	_elementBoth = _elementBoth.id
	if _elementBoth ~= nil then
		_elementBoth = _elementBoth(_self)
	end
end
local elementBoth = _elementBoth
local _self_1 = macros
local _elementOptionalId = _self_1.id
if _elementOptionalId ~= nil then
	_elementOptionalId = _elementOptionalId(_self_1)
end
local elementOptionalId = _elementOptionalId
local _self_2 = macros
local _elementOptional = _self_2.inlined
if _elementOptional ~= nil then
	_elementOptional = _elementOptional(_self_2)
end
local elementOptional = _elementOptional
local parenthesizedCallee = macros:id()
local nonNullCallee = macros:id()
local function statements()
	local _result_2 = holder
	if _result_2 ~= nil then
		_result_2 = _result_2.macros
		if _result_2 ~= nil then
			_result_2:where(32, 2, 23, "6e7076f7-17a8-52f6-b0e1-51fa6afdca83", "holder?.macros?.where()")
		end
	end
	local _result_3 = maybe
	if _result_3 ~= nil then
		_result_3:second(nil, "shared/macros@Economy")
	end
	local _result_4 = maybe
	if _result_4 ~= nil then
		_result_4 = _result_4:id("shared/macros@Economy")
	end
	if _result_4 ~= nil then
		local _result_5 = maybe
		if _result_5 ~= nil then
			_result_5 = _result_5:guard(t.interface({
				value = t.number,
				tags = t.array(t.string),
			}))
		end
		return _result_5
	end
end
return {
	statements = statements,
	fnOptional = fnOptional,
	nsReceiver = nsReceiver,
	nsCall = nsCall,
	coreGuard = coreGuard,
	coreGuardCall = coreGuardCall,
	deep = deep,
	deepBoth = deepBoth,
	deepPlain = deepPlain,
	after = after,
	coalesced = coalesced,
	inlinedFirst = inlinedFirst,
	nestedArgument = nestedArgument,
	elementCall = elementCall,
	elementPlain = elementPlain,
	elementBoth = elementBoth,
	elementOptionalId = elementOptionalId,
	elementOptional = elementOptional,
	parenthesizedCallee = parenthesizedCallee,
	nonNullCallee = nonNullCallee,
}
