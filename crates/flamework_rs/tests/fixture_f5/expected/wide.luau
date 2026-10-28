-- Compiled with roblox-ts v3.0.0
local TS = _G[script]
local Events = TS.import(script, script.Parent, "client").Events
local codec = {}
local vsize = function(n)
	return if n < 128 then 1 elseif n < 16384 then 2 elseif n < 2097152 then 3 elseif n < 268435456 then 4 else 5
end
local vwrite = function(buf, o, n_1)
	while n_1 >= 128 do
		buffer.writeu8(buf, o, n_1 % 128 + 128)
		o += 1
		n_1 = math.floor(n_1 / 128)
	end
	buffer.writeu8(buf, o, n_1)
	return o + 1
end
local vread = function(buf_1, o_1)
	local n_2 = 0
	local scale = 1
	while true do
		local b = buffer.readu8(buf_1, o_1)
		o_1 += 1
		n_2 += b % 128 * scale
		if b < 128 then
			return n_2, o_1
		end
		scale *= 128
		if scale > 268435456 then
			error("malformed payload")
		end
	end
end
codec.checkWidth = function(width, value, where, unit)
	if unit == nil then
		unit = ""
	end
	local message = `[Flamework] {width} cannot hold {value}{unit}, at {where}`
	error(message, 2)
end
-- A send whose function holds so many locals already (its parameters) that the packing, built compactly, would not
-- be smaller: the first build is kept, with its own buffer, in a do block.
local function wide(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20, p21, p22, p23, p24, p25, p26, p27, p28, p29, p30, p31, p32, p33, p34, p35, p36, p37, p38, p39, p40, p41, p42, p43, p44, p45, p46, p47, p48, p49, p50, p51, p52, p53, p54, p55, p56, p57, p58, p59, p60, p61, p62, p63, p64, p65, p66, p67, p68, p69, p70, p71, p72, p73, p74, p75, p76, p77, p78, p79, p80, p81, p82, p83, p84, p85, p86, p87, p88, p89, p90, p91, p92, p93, p94, p95, p96, p97, p98, p99, p100, p101, p102, p103, p104, p105, p106, p107, p108, p109, p110, p111, p112, p113, p114, p115, p116, p117, p118, p119, p120, p121, p122, p123, p124, p125, p126, p127, p128, p129, p130, p131, p132, p133, p134, p135, p136, p137, p138, p139, p140, p141, p142, p143, p144)
	do
		local arg = text()
		local arg_1 = p0 + p1
		local length = #arg
		local buf_2 = buffer.create(vsize(length) + length + 1)
		local o_2 = 0
		local length_1 = #arg
		o_2 = vwrite(buf_2, o_2, length_1)
		buffer.writestring(buf_2, o_2, arg)
		o_2 += length_1
		if not (arg_1 >= 0 and arg_1 <= 255 and arg_1 % 1 == 0) then
			codec.checkWidth("u8", arg_1, "'bravo' [1]")
		end
		buffer.writeu8(buf_2, o_2, arg_1)
		Events.bravo:_fire(buf_2)
	end
	return p0
end
return {
	wide = wide,
}
