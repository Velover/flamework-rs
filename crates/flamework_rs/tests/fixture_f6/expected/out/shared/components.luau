-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local SYMBOL_ATTRIBUTE_SETTER = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out", "baseComponent").SYMBOL_ATTRIBUTE_SETTER
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
local HandlerComponent
do
	local super = BaseComponent
	HandlerComponent = setmetatable({}, {
		__tostring = function()
			return "HandlerComponent"
		end,
		__index = super,
	})
	HandlerComponent.__index = HandlerComponent
	function HandlerComponent.new(...)
		local self = setmetatable({}, HandlerComponent)
		return self:constructor(...) or self
	end
	function HandlerComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) HandlerComponent metadata
		Reflect.defineMetadata(HandlerComponent, "identifier", "shared/components@HandlerComponent")
		Reflect.defineMetadata(HandlerComponent, "flamework:module", script)
		Reflect.defineMetadata(HandlerComponent, "flamework:implements", {})
		Reflect.defineMetadata(HandlerComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(HandlerComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	HandlerComponent = Component({
		tag = "FixtureHandler",
		attributes = {
			power = t.number,
		},
		instanceShape = {
			isA = { "BasePart" },
		},
	})(HandlerComponent) or HandlerComponent
end
-- Declares a tree of its own, so the structure it needs is part of every guard that names it.
local RigComponent
do
	local super = BaseComponent
	RigComponent = setmetatable({}, {
		__tostring = function()
			return "RigComponent"
		end,
		__index = super,
	})
	RigComponent.__index = RigComponent
	function RigComponent.new(...)
		local self = setmetatable({}, RigComponent)
		return self:constructor(...) or self
	end
	function RigComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) RigComponent metadata
		Reflect.defineMetadata(RigComponent, "identifier", "shared/components@RigComponent")
		Reflect.defineMetadata(RigComponent, "flamework:module", script)
		Reflect.defineMetadata(RigComponent, "flamework:implements", {})
		Reflect.defineMetadata(RigComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(RigComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	RigComponent = Component({
		tag = "FixtureRig",
		attributes = {},
		instanceShape = {
			isA = { "Model" },
			children = {
				Root = {
					isA = { "BasePart" },
				},
			},
		},
	})(RigComponent) or RigComponent
end
local LinkedComponent
do
	local super = BaseComponent
	LinkedComponent = setmetatable({}, {
		__tostring = function()
			return "LinkedComponent"
		end,
		__index = super,
	})
	LinkedComponent.__index = LinkedComponent
	function LinkedComponent.new(...)
		local self = setmetatable({}, LinkedComponent)
		return self:constructor(...) or self
	end
	function LinkedComponent:constructor(...)
		super.constructor(self, ...)
	end
	function LinkedComponent:rename()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "label", "renamed")
	end
	function LinkedComponent:accelerate()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "speed", self.attributes.speed + 1)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "speed", self.attributes.speed + 1, true)
	end
	function LinkedComponent:clear()
		self[SYMBOL_ATTRIBUTE_SETTER](self, "label", nil)
	end
	function LinkedComponent:retarget(part)
		self[SYMBOL_ATTRIBUTE_SETTER](self, "Target", part)
	end
	function LinkedComponent:reach()
		local part = self.instance.EffectHandler
		local handler = self.childComponents.EffectHandler
		local linked = self.attributeComponents.Handler
		local target = self.attributes.Target
		local raw = self.attributes.Raw
		return { part, handler, linked, target, raw }
	end
	do
		-- (Flamework) LinkedComponent metadata
		Reflect.defineMetadata(LinkedComponent, "identifier", "shared/components@LinkedComponent")
		Reflect.defineMetadata(LinkedComponent, "flamework:module", script)
		Reflect.defineMetadata(LinkedComponent, "flamework:implements", {})
		Reflect.defineMetadata(LinkedComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(LinkedComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	LinkedComponent = Component({
		tag = "FixtureLinked",
		links = { {
			kind = "attribute",
			name = "Target",
			optional = false,
			shape = {
				isA = { "BasePart" },
			},
		}, {
			kind = "attribute",
			name = "Spare",
			optional = true,
			shape = {
				isA = { "Part" },
			},
		}, {
			kind = "attribute",
			name = "Handler",
			optional = false,
			component = "shared/components@HandlerComponent",
		}, {
			kind = "attribute",
			name = "Rig",
			optional = false,
			component = "shared/components@RigComponent",
		}, {
			kind = "child",
			name = "EffectHandler",
			optional = false,
			component = "shared/components@HandlerComponent",
		}, {
			kind = "child",
			name = "SpareHandler",
			optional = false,
			component = "shared/components@HandlerComponent",
		} },
		attributes = {
			Target = t.typeof("InstanceHandle"),
			Spare = t.optional(t.typeof("InstanceHandle")),
			Handler = t.typeof("InstanceHandle"),
			Rig = t.typeof("InstanceHandle"),
			Raw = t.typeof("InstanceHandle"),
			speed = t.number,
			label = t.optional(t.string),
		},
		instanceShape = {
			isA = { "Model" },
			children = {
				Plain = {
					isA = { "BasePart" },
				},
				EffectHandler = {
					isA = { "BasePart" },
				},
				SpareHandler = {
					isA = { "BasePart" },
				},
			},
		},
	})(LinkedComponent) or LinkedComponent
end
-- Writes to an attribute through a receiver that is itself a macro call.
local CounterComponent
do
	local super = BaseComponent
	CounterComponent = setmetatable({}, {
		__tostring = function()
			return "CounterComponent"
		end,
		__index = super,
	})
	CounterComponent.__index = CounterComponent
	function CounterComponent.new(...)
		local self = setmetatable({}, CounterComponent)
		return self:constructor(...) or self
	end
	function CounterComponent:constructor(metadata, components)
		super.constructor(self, metadata)
		self.components = components
	end
	function CounterComponent:bumpPostfix(other)
		local _self = self.components:getComponent(other, "shared/components@CounterComponent")
		_self[SYMBOL_ATTRIBUTE_SETTER](_self, "count", self.components:getComponent(other, "shared/components@CounterComponent").attributes.count + 1, true)
	end
	function CounterComponent:bumpPrefix(other)
		local _self = self.components:getComponent(other, "shared/components@CounterComponent")
		_self[SYMBOL_ATTRIBUTE_SETTER](_self, "count", self.components:getComponent(other, "shared/components@CounterComponent").attributes.count + 1)
	end
	function CounterComponent:bumpCompound(other)
		local _self = self.components:getComponent(other, "shared/components@CounterComponent")
		_self[SYMBOL_ATTRIBUTE_SETTER](_self, "count", self.components:getComponent(other, "shared/components@CounterComponent").attributes.count + 1)
	end
	do
		-- (Flamework) CounterComponent metadata
		Reflect.defineMetadata(CounterComponent, "identifier", "shared/components@CounterComponent")
		Reflect.defineMetadata(CounterComponent, "flamework:module", script)
		Reflect.defineMetadata(CounterComponent, "flamework:implements", {})
		Reflect.defineMetadata(CounterComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata", "$c:components@Components" })
		Reflect.defineMetadata(CounterComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		}, {
			id = "$c:components@Components",
		} })
	end
	CounterComponent = Component({
		tag = "FixtureCounter",
		attributes = {
			count = t.number,
		},
		instanceShape = {
			isA = { "BasePart" },
		},
	})(CounterComponent) or CounterComponent
end
-- A tree three levels deep, with a child that may be one of two classes.
local DeepComponent
do
	local super = BaseComponent
	DeepComponent = setmetatable({}, {
		__tostring = function()
			return "DeepComponent"
		end,
		__index = super,
	})
	DeepComponent.__index = DeepComponent
	function DeepComponent.new(...)
		local self = setmetatable({}, DeepComponent)
		return self:constructor(...) or self
	end
	function DeepComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) DeepComponent metadata
		Reflect.defineMetadata(DeepComponent, "identifier", "shared/components@DeepComponent")
		Reflect.defineMetadata(DeepComponent, "flamework:module", script)
		Reflect.defineMetadata(DeepComponent, "flamework:implements", {})
		Reflect.defineMetadata(DeepComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(DeepComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	DeepComponent = Component({
		tag = "FixtureDeep",
		attributes = {},
		instanceShape = {
			isA = { "Model" },
			children = {
				Root = {
					isA = { "BasePart" },
					children = {
						Texture = {
							isA = { "Decal", "Texture" },
						},
					},
				},
			},
		},
	})(DeepComponent) or DeepComponent
end
--[[
	A union whose members declare children of their own has no shape: which children go with which
	class is more than a shape says, so the guard is kept for it.
]]
local EitherComponent
do
	local super = BaseComponent
	EitherComponent = setmetatable({}, {
		__tostring = function()
			return "EitherComponent"
		end,
		__index = super,
	})
	EitherComponent.__index = EitherComponent
	function EitherComponent.new(...)
		local self = setmetatable({}, EitherComponent)
		return self:constructor(...) or self
	end
	function EitherComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) EitherComponent metadata
		Reflect.defineMetadata(EitherComponent, "identifier", "shared/components@EitherComponent")
		Reflect.defineMetadata(EitherComponent, "flamework:module", script)
		Reflect.defineMetadata(EitherComponent, "flamework:implements", {})
		Reflect.defineMetadata(EitherComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(EitherComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	EitherComponent = Component({
		tag = "FixtureEither",
		attributes = {},
		instanceGuard = t.union(t.intersection(t.instanceIsA("Folder"), t.children({
			Core = t.instanceIsA("Folder"),
		})), t.intersection(t.instanceIsA("Model"), t.children({
			Root = t.instanceIsA("BasePart"),
		}))),
	})(EitherComponent) or EitherComponent
end
-- A guard written by hand is kept as it is, with no shape beside it.
local CustomGuardComponent
do
	local super = BaseComponent
	CustomGuardComponent = setmetatable({}, {
		__tostring = function()
			return "CustomGuardComponent"
		end,
		__index = super,
	})
	CustomGuardComponent.__index = CustomGuardComponent
	function CustomGuardComponent.new(...)
		local self = setmetatable({}, CustomGuardComponent)
		return self:constructor(...) or self
	end
	function CustomGuardComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) CustomGuardComponent metadata
		Reflect.defineMetadata(CustomGuardComponent, "identifier", "shared/components@CustomGuardComponent")
		Reflect.defineMetadata(CustomGuardComponent, "flamework:module", script)
		Reflect.defineMetadata(CustomGuardComponent, "flamework:implements", {})
		Reflect.defineMetadata(CustomGuardComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(CustomGuardComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	CustomGuardComponent = Component({
		tag = "FixtureCustom",
		instanceGuard = t.instanceIsA("Part"),
		attributes = {},
	})(CustomGuardComponent) or CustomGuardComponent
end
--[[
	A child naming a component with a tree of its own: the owner's shape stops at the child's class,
	and that component's tracker answers for the tree below it.
]]
local RigOwnerComponent
do
	local super = BaseComponent
	RigOwnerComponent = setmetatable({}, {
		__tostring = function()
			return "RigOwnerComponent"
		end,
		__index = super,
	})
	RigOwnerComponent.__index = RigOwnerComponent
	function RigOwnerComponent.new(...)
		local self = setmetatable({}, RigOwnerComponent)
		return self:constructor(...) or self
	end
	function RigOwnerComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) RigOwnerComponent metadata
		Reflect.defineMetadata(RigOwnerComponent, "identifier", "shared/components@RigOwnerComponent")
		Reflect.defineMetadata(RigOwnerComponent, "flamework:module", script)
		Reflect.defineMetadata(RigOwnerComponent, "flamework:implements", {})
		Reflect.defineMetadata(RigOwnerComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(RigOwnerComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	RigOwnerComponent = Component({
		tag = "FixtureRigOwner",
		links = { {
			kind = "child",
			name = "Rig",
			optional = false,
			component = "shared/components@RigComponent",
		} },
		attributes = {},
		instanceShape = {
			isA = { "Model" },
			children = {
				Rig = {
					isA = { "Model" },
				},
			},
		},
	})(RigOwnerComponent) or RigOwnerComponent
end
return {
	HandlerComponent = HandlerComponent,
	RigComponent = RigComponent,
	LinkedComponent = LinkedComponent,
	CounterComponent = CounterComponent,
	DeepComponent = DeepComponent,
	EitherComponent = EitherComponent,
	CustomGuardComponent = CustomGuardComponent,
	RigOwnerComponent = RigOwnerComponent,
}
