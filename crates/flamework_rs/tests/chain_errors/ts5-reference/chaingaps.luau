-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Flamework_1 = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Flamework
local ns = TS.import(script, game:GetService("ReplicatedStorage"), "TS", "macros")
local _macros = TS.import(script, game:GetService("ReplicatedStorage"), "TS", "macros")
local Macros = _macros.Macros
local macros = _macros.macros
local maybe = _macros.maybe
local sideEffect = _macros.sideEffect
-- Inlined and rewritten macro calls inside optional chains. TS5 replaces the call node: an inlined call's value
-- stands for the whole chain, a rewritten call keeps only its own `?.`. rbxts-rs offers these calls inside the
-- chain's nil check, where they cannot behave as in TS5, so flamework-rs refuses each with an error until it offers
-- the chain's outermost call first. TS5's output is ts5-reference/chaingaps.luau.
-- o.m?.()
local inlined0_22 = "Economy"
local inlinedValue0_23 = "shared/macros@Economy"
local inlinedValue0_24 = sideEffect()
local _implementsOf0_25 = Flamework_1._implements
if _implementsOf0_25 ~= nil then
	_implementsOf0_25 = _implementsOf0_25(macros, "shared/macros@Shop")
end
local implementsOf0_25 = _implementsOf0_25
local _implementsOf0_26 = Flamework_1._implements
if _implementsOf0_26 ~= nil then
	_implementsOf0_26 = _implementsOf0_26(sideEffect(), "shared/macros@Shop")
end
local implementsOf0_26 = _implementsOf0_26
-- o?.m()
local inlined1_22 = "Economy"
local inlinedValue1_23 = "shared/macros@Economy"
local inlinedValue1_24 = sideEffect()
local implementsOf1_25 = Flamework_1._implements(macros, "shared/macros@Shop")
local implementsOf1_26 = Flamework_1._implements(sideEffect(), "shared/macros@Shop")
-- o?.m?.()
local inlined2_22 = "Economy"
local inlinedValue2_23 = "shared/macros@Economy"
local inlinedValue2_24 = sideEffect()
local _implementsOf2_25 = Flamework_1._implements
if _implementsOf2_25 ~= nil then
	_implementsOf2_25 = _implementsOf2_25(macros, "shared/macros@Shop")
end
local implementsOf2_25 = _implementsOf2_25
local _implementsOf2_26 = Flamework_1._implements
if _implementsOf2_26 ~= nil then
	_implementsOf2_26 = _implementsOf2_26(sideEffect(), "shared/macros@Shop")
end
local implementsOf2_26 = _implementsOf2_26
-- super.m?.()
local DerivedGaps
do
	local super = Macros
	DerivedGaps = setmetatable({}, {
		__tostring = function()
			return "DerivedGaps"
		end,
		__index = super,
	})
	DerivedGaps.__index = DerivedGaps
	function DerivedGaps.new(...)
		local self = setmetatable({}, DerivedGaps)
		return self:constructor(...) or self
	end
	function DerivedGaps:constructor(...)
		super.constructor(self, ...)
	end
	function DerivedGaps:optional()
		local _exp = sideEffect()
		local _result = Flamework_1._implements
		if _result ~= nil then
			_result = _result(macros, "shared/macros@Shop")
		end
		local _result_1 = Flamework_1._implements
		if _result_1 ~= nil then
			_result_1 = _result_1(sideEffect(), "shared/macros@Shop")
		end
		return { "Economy", "shared/macros@Economy", _exp, _result, _result_1 }
	end
	function DerivedGaps:statements()
		local _ = "Economy"
		local _result = Flamework_1._implements
		if _result ~= nil then
			_result(sideEffect(), "shared/macros@Shop")
		end
	end
end
-- Function macros and core's macros.
local fnInlined = "Economy"
local nsInlined = "Economy"
local coreId = "shared/macros@Economy"
local coreIdCall = "shared/macros@Economy"
local coreIdBoth = "shared/macros@Economy"
local coreEnv = "beta"
local coreEnvFallback = "fallback"
local coreImplements = Flamework_1._implements(macros, "shared/macros@Shop")
local _coreImplementsCall = Flamework_1._implements
if _coreImplementsCall ~= nil then
	_coreImplementsCall = _coreImplementsCall(sideEffect(), "shared/macros@Shop")
end
local coreImplementsCall = _coreImplementsCall
-- Inside other expressions and chains.
local afterRewritten = Flamework_1._implements(sideEffect(), "shared/macros@Shop") == true
local _parenthesized = "Economy"
if _parenthesized ~= nil then
	_parenthesized = #_parenthesized
end
local parenthesized = _parenthesized
local asArgument = macros:second("Economy", "$p:unknown")
local _nested = maybe
if _nested ~= nil then
	_nested = _nested:second("Shop", "shared/macros@Economy")
end
local nested = _nested
local _result = maybe
if _result ~= nil then
	_result = _result:id("shared/macros@Shop")
end
local nestedValue = _result
local function statements()
	local _ = "Economy"
	Flamework_1._implements(sideEffect(), "shared/macros@Shop")
	local _1 = "shared/macros@Economy"
	local _2 = "Economy"
	if "Economy" ~= nil then
		local _result_1 = maybe
		if _result_1 ~= nil then
			_result_1 = _result_1:text("Shop")
		end
		return _result_1
	end
end
return {
	statements = statements,
	inlined0_22 = inlined0_22,
	inlinedValue0_23 = inlinedValue0_23,
	inlinedValue0_24 = inlinedValue0_24,
	implementsOf0_25 = implementsOf0_25,
	implementsOf0_26 = implementsOf0_26,
	inlined1_22 = inlined1_22,
	inlinedValue1_23 = inlinedValue1_23,
	inlinedValue1_24 = inlinedValue1_24,
	implementsOf1_25 = implementsOf1_25,
	implementsOf1_26 = implementsOf1_26,
	inlined2_22 = inlined2_22,
	inlinedValue2_23 = inlinedValue2_23,
	inlinedValue2_24 = inlinedValue2_24,
	implementsOf2_25 = implementsOf2_25,
	implementsOf2_26 = implementsOf2_26,
	DerivedGaps = DerivedGaps,
	fnInlined = fnInlined,
	nsInlined = nsInlined,
	coreId = coreId,
	coreIdCall = coreIdCall,
	coreIdBoth = coreIdBoth,
	coreEnv = coreEnv,
	coreEnvFallback = coreEnvFallback,
	coreImplements = coreImplements,
	coreImplementsCall = coreImplementsCall,
	afterRewritten = afterRewritten,
	parenthesized = parenthesized,
	asArgument = asArgument,
	nested = nested,
	nestedValue = nestedValue,
}
