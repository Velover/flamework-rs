-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local SYMBOL_ATTRIBUTE_SETTER = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out", "baseComponent").SYMBOL_ATTRIBUTE_SETTER
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
--[[
	Receivers and keys a class field initializer reassigns while the value is computed. roblox-ts counts an initializer
	as the enclosing function's code, so it holds nothing here: the setter reads the reassigned receiver and key.
]]
local FieldsComponent
do
	local super = BaseComponent
	FieldsComponent = setmetatable({}, {
		__tostring = function()
			return "FieldsComponent"
		end,
		__index = super,
	})
	FieldsComponent.__index = FieldsComponent
	function FieldsComponent.new(...)
		local self = setmetatable({}, FieldsComponent)
		return self:constructor(...) or self
	end
	function FieldsComponent:constructor(...)
		super.constructor(self, ...)
	end
	function FieldsComponent:receiver(other, spare)
		local Swapper
		do
			Swapper = setmetatable({}, {
				__tostring = function()
					return "Swapper"
				end,
			})
			Swapper.__index = Swapper
			function Swapper.new(...)
				local self = setmetatable({}, Swapper)
				return self:constructor(...) or self
			end
			function Swapper:constructor()
				other = spare
				local _ = other
				self.value = (log("field", 1))
			end
		end
		local _fn = other[SYMBOL_ATTRIBUTE_SETTER]
		local _condition = maybe("receiver")
		if _condition == nil then
			_condition = Swapper.new().value
		end
		_fn(other, "n", _condition)
	end
	function FieldsComponent:key()
		local key = "n"
		local Rekey
		do
			Rekey = setmetatable({}, {
				__tostring = function()
					return "Rekey"
				end,
			})
			Rekey.__index = Rekey
			function Rekey.new(...)
				local self = setmetatable({}, Rekey)
				return self:constructor(...) or self
			end
			function Rekey:constructor()
				key = "k"
				local _ = key
				self.value = (log("rekey", 2))
			end
		end
		local _fn = self[SYMBOL_ATTRIBUTE_SETTER]
		local _condition = maybe("key")
		if _condition == nil then
			_condition = Rekey.new().value
		end
		_fn(self, key, _condition)
	end
	function FieldsComponent:closure(other, spare)
		local swap = function()
			other = spare
			return 3
		end
		local _self = other
		_self[SYMBOL_ATTRIBUTE_SETTER](_self, "n", swap())
	end
	do
		-- (Flamework) FieldsComponent metadata
		Reflect.defineMetadata(FieldsComponent, "identifier", "shared/fields@FieldsComponent")
		Reflect.defineMetadata(FieldsComponent, "flamework:module", script)
		Reflect.defineMetadata(FieldsComponent, "flamework:implements", {})
		Reflect.defineMetadata(FieldsComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(FieldsComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	FieldsComponent = Component({
		tag = "F6Fields",
		attributes = {
			n = t.number,
			k = t.number,
		},
		instanceShape = {
			isA = { "Part" },
		},
	})(FieldsComponent) or FieldsComponent
end
return {
	FieldsComponent = FieldsComponent,
}
