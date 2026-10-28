-- Compiled with roblox-ts v3.0.0
-- An optional chain: in a function that checks the chain's start, packs, and then reads the chain again for the send,
-- as TS5 does.
local function chained(maybe, value)
	(function()
		if maybe == nil then
			return nil
		end
		local arg = value * 3
		local buf = buffer.create(8)
		buffer.writef64(buf, 0, arg)
		local _result = maybe
		if _result ~= nil then
			_result = _result.alpha
		end
		return _result:_fire(buf)
	end)();
	(function()
		if maybe == nil then
			return nil
		end
		local _result = maybe
		if _result ~= nil then
			_result = _result.charlie
		end
		return _result:_fire()
	end)()
end
return {
	chained = chained,
}
