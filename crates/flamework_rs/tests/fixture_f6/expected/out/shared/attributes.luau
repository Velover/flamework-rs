-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local SYMBOL_ATTRIBUTE_SETTER = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out", "baseComponent").SYMBOL_ATTRIBUTE_SETTER
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
-- Every write to an attribute goes through the setter.
local AttributeComponent
do
	local super = BaseComponent
	AttributeComponent = setmetatable({}, {
		__tostring = function()
			return "AttributeComponent"
		end,
		__index = super,
	})
	AttributeComponent.__index = AttributeComponent
	function AttributeComponent.new(...)
		local self = setmetatable({}, AttributeComponent)
		return self:constructor(...) or self
	end
	function AttributeComponent:constructor(...)
		super.constructor(self, ...)
	end
	function AttributeComponent:assign(part)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", 1)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "label", "text")
		self[SYMBOL_ATTRIBUTE_SETTER](self, "flag", true)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "note", nil)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "target", part)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "two words", 2)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", 3)
	end
	function AttributeComponent:compound()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count - 2)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count * 3)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count / 4)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count % 5)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count ^ 2)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.bor(self.attributes.count, 1))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.band(self.attributes.count, 3))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.bxor(self.attributes.count, 4))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.lshift(self.attributes.count, 1))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.arshift(self.attributes.count, 1))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", bit32.rshift(self.attributes.count, 1))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "label", self.attributes.label .. "suffix")
		self[SYMBOL_ATTRIBUTE_SETTER](self, "label", self.attributes.label .. tostring(self.attributes.count))
		self[SYMBOL_ATTRIBUTE_SETTER](self, "two words", self.attributes["two words"] + 7)
	end
	function AttributeComponent:logical()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "flag", self.attributes.flag or true)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "flag", self.attributes.flag and false)
		local _fn = self[SYMBOL_ATTRIBUTE_SETTER]
		local _condition = self.attributes.note
		if _condition == nil then
			_condition = "fallback"
		end
		_fn(self, "note", _condition)
	end
	function AttributeComponent:updates()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1, true)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count - 1, true)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count - 1)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "two words", self.attributes["two words"] + 1, true)
	end
	function AttributeComponent:values()
		local assigned = (self[SYMBOL_ATTRIBUTE_SETTER](self, "count", 10))
		local added = (self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 5))
		local post = self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1, true)
		local pre = self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1)
		local postDown = self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count - 1, true)
		local preDown = self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count - 1)
		local removed = self[SYMBOL_ATTRIBUTE_SETTER](self, "note", nil)
		return { assigned, added, post, pre, postDown, preDown, removed }
	end
	function AttributeComponent:removes()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "note", nil)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "note", nil)
	end
	function AttributeComponent:effects()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", sideEffect())
		self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + sideEffect())
		self[SYMBOL_ATTRIBUTE_SETTER](self, pickKey(), 4)
	end
	function AttributeComponent:nested()
		local run = function()
			self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1)
		end
		run()
		for i = 0, 2 do
			self[SYMBOL_ATTRIBUTE_SETTER](self, "count", self.attributes.count + 1, true)
		end
		if self.attributes.flag then
			self[SYMBOL_ATTRIBUTE_SETTER](self, "label", "flagged")
		end
	end
	function AttributeComponent:other(other, components, part)
		other[SYMBOL_ATTRIBUTE_SETTER](other, "count", 1)
		other[SYMBOL_ATTRIBUTE_SETTER](other, "count", other.attributes.count + 1, true)
		local _self = components:getComponent(part, "shared/attributes@AttributeComponent")
		_self[SYMBOL_ATTRIBUTE_SETTER](_self, "label", "remote")
		local _self_1 = components:getComponent(part, "shared/attributes@AttributeComponent")
		_self_1[SYMBOL_ATTRIBUTE_SETTER](_self_1, "note", nil)
	end
	function AttributeComponent:notWrites()
		(self.attributes).count = 1
		self.attributes.count = 2
		self.attributes.count += 1
		self.attributes.count = 3
		local attributes = self.attributes
		attributes.count = 4
	end
	function AttributeComponent:reads()
		local count = self.attributes.count
		local label = self.attributes.label
		return { count, label }
	end
	do
		-- (Flamework) AttributeComponent metadata
		Reflect.defineMetadata(AttributeComponent, "identifier", "shared/attributes@AttributeComponent")
		Reflect.defineMetadata(AttributeComponent, "flamework:module", script)
		Reflect.defineMetadata(AttributeComponent, "flamework:implements", {})
		Reflect.defineMetadata(AttributeComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(AttributeComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	AttributeComponent = Component({
		tag = "F6Attributes",
		links = { {
			kind = "attribute",
			name = "target",
			optional = true,
			shape = {
				isA = { "BasePart" },
			},
		} },
		attributes = {
			count = t.number,
			label = t.string,
			flag = t.boolean,
			note = t.optional(t.string),
			["two words"] = t.number,
			target = t.optional(t.typeof("InstanceHandle")),
		},
		instanceShape = {
			isA = { "BasePart" },
		},
	})(AttributeComponent) or AttributeComponent
end
return {
	AttributeComponent = AttributeComponent,
}
