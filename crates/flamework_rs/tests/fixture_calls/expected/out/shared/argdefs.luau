-- Compiled with roblox-ts v3.0.0
-- @metadata macro
local function wrap(inner, meta)
	return { inner, meta }
end
-- @metadata macro
local function pre(meta, inner)
	return { meta, inner }
end
-- @metadata macro
local function two(first, written, second)
	return { first, written, second }
end
return {
	wrap = wrap,
	pre = pre,
	two = two,
}
