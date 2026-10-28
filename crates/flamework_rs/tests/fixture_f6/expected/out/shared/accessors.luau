-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
-- A getter named `attributes`, last and with no links: the TS drops it for the generated guards.
local GetterAttributes
do
	local super = BaseComponent
	GetterAttributes = setmetatable({}, {
		__tostring = function()
			return "GetterAttributes"
		end,
		__index = super,
	})
	GetterAttributes.__index = GetterAttributes
	function GetterAttributes.new(...)
		local self = setmetatable({}, GetterAttributes)
		return self:constructor(...) or self
	end
	function GetterAttributes:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) GetterAttributes metadata
		Reflect.defineMetadata(GetterAttributes, "identifier", "shared/accessors@GetterAttributes")
		Reflect.defineMetadata(GetterAttributes, "flamework:module", script)
		Reflect.defineMetadata(GetterAttributes, "flamework:implements", {})
		Reflect.defineMetadata(GetterAttributes, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(GetterAttributes, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	GetterAttributes = Component({
		tag = "F6Getter",
		attributes = {
			name = t.string,
		},
		instanceShape = {
			isA = { "Part" },
		},
	})(GetterAttributes) or GetterAttributes
end
-- A setter named `attributes`: dropped the same way.
local SetterAttributes
do
	local super = BaseComponent
	SetterAttributes = setmetatable({}, {
		__tostring = function()
			return "SetterAttributes"
		end,
		__index = super,
	})
	SetterAttributes.__index = SetterAttributes
	function SetterAttributes.new(...)
		local self = setmetatable({}, SetterAttributes)
		return self:constructor(...) or self
	end
	function SetterAttributes:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) SetterAttributes metadata
		Reflect.defineMetadata(SetterAttributes, "identifier", "shared/accessors@SetterAttributes")
		Reflect.defineMetadata(SetterAttributes, "flamework:module", script)
		Reflect.defineMetadata(SetterAttributes, "flamework:implements", {})
		Reflect.defineMetadata(SetterAttributes, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(SetterAttributes, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	SetterAttributes = Component({
		tag = "F6Setter",
		attributes = {
			name = t.string,
		},
		instanceShape = {
			isA = { "Part" },
		},
	})(SetterAttributes) or SetterAttributes
end
return {
	GetterAttributes = GetterAttributes,
	SetterAttributes = SetterAttributes,
}
