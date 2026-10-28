-- Compiled with roblox-ts v3.0.0
local TS = require(game:GetService("ReplicatedStorage"):WaitForChild("rbxts_include"):WaitForChild("RuntimeLib"))
local Reflect = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "core", "out").Reflect
local t = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@rbxts", "t", "lib", "ts").t
local _components = TS.import(script, game:GetService("ReplicatedStorage"), "rbxts_include", "node_modules", "@flamework-experimental", "components", "out")
local BaseComponent = _components.BaseComponent
local Component = _components.Component
-- An instance type whose child may be another of itself, linked from an attribute.
local RecursiveLink
do
	local super = BaseComponent
	RecursiveLink = setmetatable({}, {
		__tostring = function()
			return "RecursiveLink"
		end,
		__index = super,
	})
	RecursiveLink.__index = RecursiveLink
	function RecursiveLink.new(...)
		local self = setmetatable({}, RecursiveLink)
		return self:constructor(...) or self
	end
	function RecursiveLink:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) RecursiveLink metadata
		Reflect.defineMetadata(RecursiveLink, "identifier", "shared/recursiveLink@RecursiveLink")
		Reflect.defineMetadata(RecursiveLink, "flamework:module", script)
		Reflect.defineMetadata(RecursiveLink, "flamework:implements", {})
		Reflect.defineMetadata(RecursiveLink, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(RecursiveLink, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	RecursiveLink = Component({
		tag = "RecursiveLink",
		links = { {
			kind = "attribute",
			name = "branch",
			optional = false,
			guard = (function()
				local depth = 0
				local memo
				local Branch_1
				Branch_1 = t.intersection(t.instanceIsA("Folder"), t.children({
					Twig = t.optional(function(value)
						if depth >= 1000 then
							return false
						end
						local results
						if type(value) == "table" then
							results = memo[Branch_1]
							if results == nil then
								results = {}
								memo[Branch_1] = results
							else
								local known = results[value]
								if known ~= nil then
									return known
								end
							end
						end
						depth += 1
						local result = Branch_1(value)
						depth -= 1
						if results ~= nil then
							results[value] = result
						end
						return result
					end),
				}))
				return function(value_1)
					local outerDepth = depth
					local outerMemo = memo
					memo = {}
					local ok, result_1 = pcall(Branch_1, value_1)
					depth = outerDepth
					memo = outerMemo
					if not ok then
						error(result_1, 0)
					end
					return result_1
				end
			end)(),
		} },
		attributes = {
			branch = t.typeof("InstanceHandle"),
		},
		instanceShape = {
			isA = { "Instance" },
		},
	})(RecursiveLink) or RecursiveLink
end
-- A recursive tree behind an optional attribute, and a component linked twice.
local RecursiveTwice
do
	local super = BaseComponent
	RecursiveTwice = setmetatable({}, {
		__tostring = function()
			return "RecursiveTwice"
		end,
		__index = super,
	})
	RecursiveTwice.__index = RecursiveTwice
	function RecursiveTwice.new(...)
		local self = setmetatable({}, RecursiveTwice)
		return self:constructor(...) or self
	end
	function RecursiveTwice:constructor(...)
		super.constructor(self, ...)
	end
	do
		-- (Flamework) RecursiveTwice metadata
		Reflect.defineMetadata(RecursiveTwice, "identifier", "shared/recursiveLink@RecursiveTwice")
		Reflect.defineMetadata(RecursiveTwice, "flamework:module", script)
		Reflect.defineMetadata(RecursiveTwice, "flamework:implements", {})
		Reflect.defineMetadata(RecursiveTwice, "flamework:parameters", { "$c:baseComponent@ComponentMetadata" })
		Reflect.defineMetadata(RecursiveTwice, "flamework:dependencies", { {
			id = "$c:baseComponent@ComponentMetadata",
		} })
	end
	RecursiveTwice = Component({
		tag = "F6RecursiveTwice",
		links = { {
			kind = "attribute",
			name = "first",
			optional = true,
			guard = (function()
				local depth_1 = 0
				local memo_1
				local Branch_2
				Branch_2 = t.intersection(t.instanceIsA("Folder"), t.children({
					Twig = t.optional(function(value_2)
						if depth_1 >= 1000 then
							return false
						end
						local results_1
						if type(value_2) == "table" then
							results_1 = memo_1[Branch_2]
							if results_1 == nil then
								results_1 = {}
								memo_1[Branch_2] = results_1
							else
								local known_1 = results_1[value_2]
								if known_1 ~= nil then
									return known_1
								end
							end
						end
						depth_1 += 1
						local result_2 = Branch_2(value_2)
						depth_1 -= 1
						if results_1 ~= nil then
							results_1[value_2] = result_2
						end
						return result_2
					end),
				}))
				return function(value_3)
					local outerDepth_1 = depth_1
					local outerMemo_1 = memo_1
					memo_1 = {}
					local ok_1, result_3 = pcall(Branch_2, value_3)
					depth_1 = outerDepth_1
					memo_1 = outerMemo_1
					if not ok_1 then
						error(result_3, 0)
					end
					return result_3
				end
			end)(),
		}, {
			kind = "attribute",
			name = "second",
			optional = false,
			component = "shared/recursiveLink@RecursiveLink",
		}, {
			kind = "attribute",
			name = "third",
			optional = true,
			component = "shared/recursiveLink@RecursiveLink",
		} },
		attributes = {
			first = t.optional(t.typeof("InstanceHandle")),
			second = t.typeof("InstanceHandle"),
			third = t.optional(t.typeof("InstanceHandle")),
		},
		instanceShape = {
			isA = { "Instance" },
		},
	})(RecursiveTwice) or RecursiveTwice
end
return {
	RecursiveLink = RecursiveLink,
	RecursiveTwice = RecursiveTwice,
}
