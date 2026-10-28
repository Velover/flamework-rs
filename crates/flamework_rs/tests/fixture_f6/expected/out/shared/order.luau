-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local SYMBOL_ATTRIBUTE_SETTER = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out", "baseComponent").SYMBOL_ATTRIBUTE_SETTER
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
-- Shorthand fields around a written `attributes` the TS moves.
local tag = "F6Shorthand"
local attributes = {
	name = t.string,
}
local defaults = {
	count = 5,
}
local ShorthandComponent
do
	local super = BaseComponent
	ShorthandComponent = setmetatable({}, {
		__tostring = function()
			return "ShorthandComponent"
		end,
		__index = super,
	})
	ShorthandComponent.__index = ShorthandComponent
	function ShorthandComponent.new(...)
		local self = setmetatable({}, ShorthandComponent)
		return self:constructor(...) or self
	end
	function ShorthandComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) ShorthandComponent metadata
		Reflect.defineMetadata(ShorthandComponent, "identifier", "shared/order@ShorthandComponent")
		Reflect.defineMetadata(ShorthandComponent, "flamework:module", script)
		Reflect.defineMetadata(ShorthandComponent, "flamework:implements", {})
		Reflect.defineMetadata(ShorthandComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(ShorthandComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	ShorthandComponent = Component({
		tag = tag,
		defaults = defaults,
		attributes = {
			name = t.string,
			count = t.number,
		},
		instanceShape = {
			isA = { "Part" },
		},
	})(ShorthandComponent) or ShorthandComponent
end
-- The TS evaluates the fields as it emits them: `tag`, `defaults`, then `attributes`.
local MovedComponent
do
	local super = BaseComponent
	MovedComponent = setmetatable({}, {
		__tostring = function()
			return "MovedComponent"
		end,
		__index = super,
	})
	MovedComponent.__index = MovedComponent
	function MovedComponent.new(...)
		local self = setmetatable({}, MovedComponent)
		return self:constructor(...) or self
	end
	function MovedComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) MovedComponent metadata
		Reflect.defineMetadata(MovedComponent, "identifier", "shared/order@MovedComponent")
		Reflect.defineMetadata(MovedComponent, "flamework:module", script)
		Reflect.defineMetadata(MovedComponent, "flamework:implements", {})
		Reflect.defineMetadata(MovedComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(MovedComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	local _object = {
		tag = log("m.tag", "F6Moved"),
	}
	local _left = "defaults"
	local _object_1 = {}
	local _left_1 = "count"
	local _condition = maybe("m.def")
	if _condition == nil then
		_condition = 1
	end
	_object_1[_left_1] = _condition
	_object[_left] = _object_1
	_object.attributes = {
		name = log("m.attr", t.string),
		count = t.number,
	}
	_object.instanceShape = {
		isA = { "Part" },
	}
	MovedComponent = Component(_object)(MovedComponent) or MovedComponent
end
-- `attributes` already last, but its value needs statements: they run after the fields before it.
local LastComponent
do
	local super = BaseComponent
	LastComponent = setmetatable({}, {
		__tostring = function()
			return "LastComponent"
		end,
		__index = super,
	})
	LastComponent.__index = LastComponent
	function LastComponent.new(...)
		local self = setmetatable({}, LastComponent)
		return self:constructor(...) or self
	end
	function LastComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) LastComponent metadata
		Reflect.defineMetadata(LastComponent, "identifier", "shared/order@LastComponent")
		Reflect.defineMetadata(LastComponent, "flamework:module", script)
		Reflect.defineMetadata(LastComponent, "flamework:implements", {})
		Reflect.defineMetadata(LastComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(LastComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	local _object = {
		tag = log("l.tag", "F6Last"),
	}
	local _left = "attributes"
	local _object_1 = {}
	local _left_1 = "name"
	local _condition = maybe("l.attr")
	if _condition == nil then
		_condition = 0
	end
	_object_1[_left_1] = if _condition > 0 then t.string else t.string
	_object[_left] = _object_1
	_object.instanceShape = {
		isA = { "Part" },
	}
	LastComponent = Component(_object)(LastComponent) or LastComponent
end
-- A computed name: the fields are still evaluated as the TS emits them.
local ComputedComponent
do
	local super = BaseComponent
	ComputedComponent = setmetatable({}, {
		__tostring = function()
			return "ComputedComponent"
		end,
		__index = super,
	})
	ComputedComponent.__index = ComputedComponent
	function ComputedComponent.new(...)
		local self = setmetatable({}, ComputedComponent)
		return self:constructor(...) or self
	end
	function ComputedComponent:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) ComputedComponent metadata
		Reflect.defineMetadata(ComputedComponent, "identifier", "shared/order@ComputedComponent")
		Reflect.defineMetadata(ComputedComponent, "flamework:module", script)
		Reflect.defineMetadata(ComputedComponent, "flamework:implements", {})
		Reflect.defineMetadata(ComputedComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(ComputedComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	local _object = {
		tag = log("c.tag", "F6Computed"),
	}
	local _left = "defaults"
	local _object_1 = {}
	local _left_1 = "count"
	local _condition = maybe("c.def")
	if _condition == nil then
		_condition = 2
	end
	_object_1[_left_1] = _condition
	_object[_left] = _object_1
	_object.attributes = {
		name = t.string,
		count = t.number,
	}
	_object.instanceShape = {
		isA = { "Part" },
	}
	ComputedComponent = Component(_object)(ComputedComponent) or ComputedComponent
end
-- A receiver and a key that a closure reassigns while the value is computed.
local RebindingComponent
do
	local super = BaseComponent
	RebindingComponent = setmetatable({}, {
		__tostring = function()
			return "RebindingComponent"
		end,
		__index = super,
	})
	RebindingComponent.__index = RebindingComponent
	function RebindingComponent.new(...)
		local self = setmetatable({}, RebindingComponent)
		return self:constructor(...) or self
	end
	function RebindingComponent:constructor(...)
		super.constructor(self, ...)
	end
	function RebindingComponent:rebind(other, spare)
		local swap = function()
			other = spare
			return log("swap", 1)
		end
		local _self = other
		local _fn = _self[SYMBOL_ATTRIBUTE_SETTER]
		local _condition = maybe("rb")
		if _condition == nil then
			_condition = swap()
		end
		_fn(_self, "n", _condition)
		local key = "n"
		local rekey = function()
			key = "k"
			return log("rekey", 2)
		end
		local _fn_1 = self[SYMBOL_ATTRIBUTE_SETTER]
		local _exp = key
		local _condition_1 = maybe("rk")
		if _condition_1 == nil then
			_condition_1 = rekey()
		end
		_fn_1(self, _exp, _condition_1)
		local target = other
		local _self_1 = target
		local _fn_2 = _self_1[SYMBOL_ATTRIBUTE_SETTER]
		local _result
		if maybe("tc") ~= nil then
			target = spare
			local _ = target
			_result = 5
		else
			_result = 6
		end
		_fn_2(_self_1, "n", _result)
	end
	function RebindingComponent:untouched(other)
		local _fn = other[SYMBOL_ATTRIBUTE_SETTER]
		local _condition = maybe("u")
		if _condition == nil then
			_condition = 3
		end
		_fn(other, "n", _condition)
		local _fn_1 = self[SYMBOL_ATTRIBUTE_SETTER]
		local _condition_1 = maybe("v")
		if _condition_1 == nil then
			_condition_1 = 4
		end
		_fn_1(self, "k", _condition_1)
	end
	do
		-- (Flamework) RebindingComponent metadata
		Reflect.defineMetadata(RebindingComponent, "identifier", "shared/order@RebindingComponent")
		Reflect.defineMetadata(RebindingComponent, "flamework:module", script)
		Reflect.defineMetadata(RebindingComponent, "flamework:implements", {})
		Reflect.defineMetadata(RebindingComponent, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(RebindingComponent, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	RebindingComponent = Component({
		tag = "F6Rebinding",
		attributes = {
			n = t.number,
			k = t.number,
		},
		instanceShape = {
			isA = { "Part" },
		},
	})(RebindingComponent) or RebindingComponent
end
return {
	ShorthandComponent = ShorthandComponent,
	MovedComponent = MovedComponent,
	LastComponent = LastComponent,
	ComputedComponent = ComputedComponent,
	RebindingComponent = RebindingComponent,
}
