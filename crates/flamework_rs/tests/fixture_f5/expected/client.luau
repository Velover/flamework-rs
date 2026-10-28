-- Compiled with roblox-ts v3.0.0
local TS = _G[script]
local t = TS.import(script, TS.getModule(script, "@rbxts", "t").lib.ts).t
local _network = TS.import(script, script.Parent, "network")
local GlobalEvents = _network.GlobalEvents
local GlobalFunctions = _network.GlobalFunctions
local vsize = function(n)
	return if n < 128 then 1 elseif n < 16384 then 2 elseif n < 2097152 then 3 elseif n < 268435456 then 4 else 5
end
local vwrite = function(buf_2, o_1, n_1)
	while n_1 >= 128 do
		buffer.writeu8(buf_2, o_1, n_1 % 128 + 128)
		o_1 += 1
		n_1 = math.floor(n_1 / 128)
	end
	buffer.writeu8(buf_2, o_1, n_1)
	return o_1 + 1
end
local vread = function(buf_3, o_2)
	local n_2 = 0
	local scale = 1
	while true do
		local b = buffer.readu8(buf_3, o_2)
		o_2 += 1
		n_2 += b % 128 * scale
		if b < 128 then
			return n_2, o_2
		end
		scale *= 128
		if scale > 268435456 then
			error("malformed payload")
		end
	end
end
local Events = GlobalEvents:createClient({}, {
	incomingIds = { "alpha", "lima", "mike" },
	incoming = {
		alpha = { { t.number }, nil },
		lima = { { t.string }, nil },
		mike = { { t.number, t.array(t.boolean) }, nil },
	},
	incomingUnreliable = {},
	outgoingIds = { "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "letters" },
	outgoingUnreliable = {
		golf = true,
	},
	incomingSerializers = {
		alpha = (function(buf_4)
			local value_1 = { buffer.readf64(buf_4, 0) }
			if buffer.len(buf_4) ~= 8 then
				error("malformed payload")
			end
			return value_1
		end),
		lima = (function(buf_5)
			local o_3 = 0
			local length_2, o_4 = vread(buf_5, o_3)
			local text_2 = buffer.readstring(buf_5, o_4, length_2)
			o_3 = o_4 + length_2
			local value_2 = { text_2 }
			if o_3 ~= buffer.len(buf_5) then
				error("malformed payload")
			end
			return value_2
		end),
		mike = (function(buf_6)
			local o_5 = 0
			local arg_1 = buffer.readf64(buf_6, o_5)
			local count, o_6 = vread(buf_6, o_5 + 8)
			o_5 = o_6
			if count > buffer.len(buf_6) - o_5 then
				error("malformed payload")
			end
			local array = table.create(count)
			for i_1 = 1, count do
				array[i_1] = buffer.readu8(buf_6, o_5) ~= 0
				o_5 += 1
			end
			local value_3 = { arg_1, array }
			if o_5 ~= buffer.len(buf_6) then
				error("malformed payload")
			end
			return value_3
		end),
	},
	namespaceIds = { "juliet" },
	namespaces = {
		juliet = {
			incomingIds = {},
			incoming = {},
			incomingUnreliable = {},
			outgoingIds = { "kilo" },
			outgoingUnreliable = {},
			incomingSerializers = {},
			namespaceIds = {},
			namespaces = {},
		},
	},
})
local Functions = GlobalFunctions:createClient({}, {
	incomingIds = { "romeo" },
	incoming = {
		romeo = { { t.number }, nil },
	},
	outgoingIds = { "november", "oscar", "papa", "quebec" },
	outgoing = {
		november = t.number,
		oscar = t.string,
		papa = t.none,
		quebec = t.optional(t.interface({
			x = t.number,
			y = t.number,
		})),
	},
	incomingSerializers = {
		romeo = (function(buf_7)
			local value_4 = { buffer.readf64(buf_7, 0) }
			if buffer.len(buf_7) ~= 8 then
				error("malformed payload")
			end
			return value_4
		end),
	},
	outgoingResults = {
		november = (function(buf_8)
			local value_5 = { buffer.readf64(buf_8, 0) }
			if buffer.len(buf_8) ~= 8 then
				error("malformed payload")
			end
			return value_5
		end),
		oscar = (function(buf_9)
			local o_7 = 0
			local length_3, o_8 = vread(buf_9, o_7)
			local text_3 = buffer.readstring(buf_9, o_8, length_3)
			o_7 = o_8 + length_3
			local value_6 = { text_3 }
			if o_7 ~= buffer.len(buf_9) then
				error("malformed payload")
			end
			return value_6
		end),
		quebec = (function(buf_10)
			local o_9 = 0
			local present = buffer.readu8(buf_10, o_9) ~= 0
			local value_7
			o_9 += 1
			if present then
				local x_1 = buffer.readf64(buf_10, o_9)
				local y_1 = buffer.readf64(buf_10, o_9 + 8)
				value_7 = {
					x = x_1,
					y = y_1,
				}
				o_9 += 16
			end
			local value_8 = { value_7 }
			if o_9 ~= buffer.len(buf_10) then
				error("malformed payload")
			end
			return value_8
		end),
	},
	namespaceIds = {},
	namespaces = {},
})
local codec = {}
codec.checkWidth = function(width, value_9, where, unit)
	if unit == nil then
		unit = ""
	end
	local message = `[Flamework] {width} cannot hold {value_9}{unit}, at {where}`
	error(message, 2)
end
-- Arguments read where they are (identifiers, literals, parameters) and bound (anything else); a target that is
-- more than a plain read is bound first.
local function shapes(value, coord, part)
	local buf_11 = buffer.create(8)
	buffer.writef64(buf_11, 0, value)
	Events.alpha:_fire(buf_11)
	local buf_12 = buffer.create(8)
	buffer.writef64(buf_12, 0, 3)
	Events.alpha:_fire(buf_12)
	local arg_2 = compute()
	local buf_13 = buffer.create(8)
	buffer.writef64(buf_13, 0, arg_2)
	Events.alpha:_fire(buf_13)
	local length_4 = #"name"
	local buf_14 = buffer.create(vsize(length_4) + length_4 + 1)
	local o_10 = 0
	local text_4 = "name"
	local length_5 = #text_4
	o_10 = vwrite(buf_14, o_10, length_5)
	buffer.writestring(buf_14, o_10, text_4)
	o_10 += length_5
	buffer.writeu8(buf_14, o_10, 7)
	Events.bravo:_fire(buf_14)
	local arg_3 = `n{value}`
	local length_6 = #arg_3
	local buf_15 = buffer.create(vsize(length_6) + length_6 + 1)
	local o_11 = 0
	local length_7 = #arg_3
	o_11 = vwrite(buf_15, o_11, length_7)
	buffer.writestring(buf_15, o_11, arg_3)
	o_11 += length_7
	if not (value >= 0 and value <= 255 and value % 1 == 0) then
		codec.checkWidth("u8", value, "'bravo' [1]")
	end
	buffer.writeu8(buf_15, o_11, value)
	Events.bravo:_fire(buf_15)
	Events.charlie:_fire()
	local buf_16 = buffer.create(17)
	buffer.writef64(buf_16, 0, coord.x)
	buffer.writef64(buf_16, 8, coord.y)
	buffer.writeu8(buf_16, 16, 0)
	Events.delta:_fire(buf_16)
	local arg_4 = {
		x = value,
		y = 2,
	}
	local v = "tag"
	local size_1 = 1
	if v ~= nil then
		local length_8 = #v
		size_1 += vsize(length_8) + length_8
	end
	local buf_17 = buffer.create(size_1 + 16)
	local o_12 = 0
	buffer.writef64(buf_17, o_12, arg_4.x)
	buffer.writef64(buf_17, o_12 + 8, arg_4.y)
	local v_1 = "tag"
	buffer.writeu8(buf_17, o_12 + 16, if v_1 ~= nil then 1 else 0)
	o_12 += 17
	if v_1 ~= nil then
		local length_9 = #v_1
		o_12 = vwrite(buf_17, o_12, length_9)
		buffer.writestring(buf_17, o_12, v_1)
		o_12 += length_9
	end
	Events.delta:_fire(buf_17)
	local buf_18 = buffer.create(1)
	buffer.writeu8(buf_18, 0, 0)
	Events.echo:_fire(buf_18)
	local buf_19 = buffer.create(25)
	buffer.writeu8(buf_19, 0, 3)
	buffer.writef64(buf_19, 1, 1)
	buffer.writef64(buf_19, 9, 2)
	buffer.writef64(buf_19, 17, value)
	Events.echo:_fire(buf_19)
	local arg_5 = compute()
	local length_10 = #"first"
	local buf_20 = buffer.create(vsize(length_10) + length_10 + 17)
	local o_13 = 0
	local text_5 = "first"
	local length_11 = #text_5
	o_13 = vwrite(buf_20, o_13, length_11)
	buffer.writestring(buf_20, o_13, text_5)
	o_13 += length_11
	buffer.writeu8(buf_20, o_13, 2)
	buffer.writef64(buf_20, o_13 + 1, 1)
	buffer.writef64(buf_20, o_13 + 9, arg_5)
	Events.foxtrot:_fire(buf_20)
	local buf_21 = buffer.create(8)
	buffer.writef64(buf_21, 0, value)
	Events.golf:_fire(buf_21)
	Events.hotel:fire(value)
	local length_12 = #"label"
	local buf_22 = buffer.create(vsize(length_12) + length_12 + 4)
	local blobs = {}
	local o_14 = 0
	local blob = part
	if blob ~= nil then
		table.insert(blobs, blob)
		buffer.writeu32(buf_22, o_14, #blobs)
	else
		buffer.writeu32(buf_22, o_14, 0)
	end
	local text_6 = "label"
	local length_13 = #text_6
	o_14 = vwrite(buf_22, o_14 + 4, length_13)
	buffer.writestring(buf_22, o_14, text_6)
	o_14 += length_13
	Events.india:_fire(buf_22, blobs)
	local buf_23 = buffer.create(1)
	buffer.writeu8(buf_23, 0, if true then 1 else 0)
	Events.juliet.kilo:_fire(buf_23)
	local arg_6 = value + 1
	local buf_24 = buffer.create(8)
	buffer.writef64(buf_24, 0, arg_6)
	Events.alpha:_fire(buf_24)
	local target = pick().alpha
	local buf_25 = buffer.create(8)
	buffer.writef64(buf_25, 0, value)
	target:_fire(buf_25)
	local length_14 = #"x"
	local buf_26 = buffer.create(vsize(length_14) + length_14 + 1)
	local o_15 = 0
	local text_7 = "x"
	local length_15 = #text_7
	o_15 = vwrite(buf_26, o_15, length_15)
	buffer.writestring(buf_26, o_15, text_7)
	o_15 += length_15
	buffer.writeu8(buf_26, o_15, 1)
	handlers[1].bravo:_fire(buf_26)
	local alias = Events
	local buf_27 = buffer.create(8)
	buffer.writef64(buf_27, 0, value)
	alias.alpha:_fire(buf_27)
end
-- Spread arguments: the list is gathered first, as roblox-ts gathers an array literal with spreads.
local function spreads(values, more)
	local _array = {}
	local _length = #_array
	table.move(values, 1, #values, _length + 1, _array)
	local args = _array
	local count_1 = #args
	local buf_28 = buffer.create(vsize(count_1) + count_1 * 8)
	local o_16 = 0
	local count_2 = #args
	o_16 = vwrite(buf_28, o_16, count_2)
	for i_2 = 1, count_2 do
		local item_1 = args[i_2]
		if item_1 == nil then
			error(`[Flamework] the argument list has no value at 'echo' [{i_2 - 1}]`)
		end
		buffer.writef64(buf_28, o_16, item_1)
		o_16 += 8
	end
	Events.echo:_fire(buf_28)
	local _array_1 = { 1 }
	local _length_1 = #_array_1
	table.move(values, 1, #values, _length_1 + 1, _array_1)
	local args_1 = _array_1
	local count_3 = #args_1
	local buf_29 = buffer.create(vsize(count_3) + count_3 * 8)
	local o_17 = 0
	local count_4 = #args_1
	o_17 = vwrite(buf_29, o_17, count_4)
	for i_3 = 1, count_4 do
		local item_2 = args_1[i_3]
		if item_2 == nil then
			error(`[Flamework] the argument list has no value at 'echo' [{i_3 - 1}]`)
		end
		buffer.writef64(buf_29, o_17, item_2)
		o_17 += 8
	end
	Events.echo:_fire(buf_29)
	local _array_2 = {}
	local _length_2 = #_array_2
	local _valuesLength = #values
	table.move(values, 1, _valuesLength, _length_2 + 1, _array_2)
	_length_2 += _valuesLength
	_array_2[_length_2 + 1] = 2
	local args_2 = _array_2
	local count_5 = #args_2
	local buf_30 = buffer.create(vsize(count_5) + count_5 * 8)
	local o_18 = 0
	local count_6 = #args_2
	o_18 = vwrite(buf_30, o_18, count_6)
	for i_4 = 1, count_6 do
		local item_3 = args_2[i_4]
		if item_3 == nil then
			error(`[Flamework] the argument list has no value at 'echo' [{i_4 - 1}]`)
		end
		buffer.writef64(buf_30, o_18, item_3)
		o_18 += 8
	end
	Events.echo:_fire(buf_30)
	local _array_3 = {}
	local _length_3 = #_array_3
	local _valuesLength_1 = #values
	table.move(values, 1, _valuesLength_1, _length_3 + 1, _array_3)
	_length_3 += _valuesLength_1
	table.move(more, 1, #more, _length_3 + 1, _array_3)
	local args_3 = _array_3
	local count_7 = #args_3
	local buf_31 = buffer.create(vsize(count_7) + count_7 * 8)
	local o_19 = 0
	local count_8 = #args_3
	o_19 = vwrite(buf_31, o_19, count_8)
	for i_5 = 1, count_8 do
		local item_4 = args_3[i_5]
		if item_4 == nil then
			error(`[Flamework] the argument list has no value at 'echo' [{i_5 - 1}]`)
		end
		buffer.writef64(buf_31, o_19, item_4)
		o_19 += 8
	end
	Events.echo:_fire(buf_31)
	local _array_4 = { "first" }
	local _length_4 = #_array_4
	table.move(values, 1, #values, _length_4 + 1, _array_4)
	local args_4 = _array_4
	local length_16 = #(args_4[1])
	local count_9 = math.max(#args_4 - 1, 0)
	local buf_32 = buffer.create(vsize(length_16) + length_16 + (vsize(count_9) + count_9 * 8))
	local o_20 = 0
	local text_8 = args_4[1]
	local length_17 = #text_8
	o_20 = vwrite(buf_32, o_20, length_17)
	buffer.writestring(buf_32, o_20, text_8)
	o_20 += length_17
	local count_10 = math.max(#args_4 - 1, 0)
	o_20 = vwrite(buf_32, o_20, count_10)
	for i_6 = 2, count_10 + 1 do
		local item_5 = args_4[i_6]
		if item_5 == nil then
			error(`[Flamework] the argument list has no value at 'foxtrot' [{i_6 - 1}]`)
		end
		buffer.writef64(buf_32, o_20, item_5)
		o_20 += 8
	end
	Events.foxtrot:_fire(buf_32)
end
-- Where the call runs: ahead of its statement when that runs it exactly when the call runs, in a function otherwise.
local function positions(flag, items)
	local sent = flag and (function()
		local arg_9 = compute()
		local buf_35 = buffer.create(8)
		buffer.writef64(buf_35, 0, arg_9)
		return Events.alpha:_fire(buf_35)
	end)()
	local picked = if #items > 0 then (function()
		local arg_10 = items[1]
		local buf_36 = buffer.create(8)
		buffer.writef64(buf_36, 0, arg_10)
		return Events.alpha:_fire(buf_36)
	end)() else nil
	local list = { #items, (function()
		local arg_11 = #items
		local buf_37 = buffer.create(8)
		buffer.writef64(buf_37, 0, arg_11)
		return Events.alpha:_fire(buf_37)
	end)() }
	local i = 0
	while true do
		local _condition = i < 3
		if _condition then
			(function()
				local buf_38 = buffer.create(8)
				buffer.writef64(buf_38, 0, i)
				return Events.alpha:_fire(buf_38)
			end)()
			_condition = true
		end
		if not _condition then
			break
		end
		i += 1
	end
	for j = 0, 1 do
		local arg_7 = j * 2
		local buf_33 = buffer.create(8)
		buffer.writef64(buf_33, 0, arg_7)
		Events.alpha:_fire(buf_33)
	end
	if flag then
		local arg_8 = compute()
		local buf_34 = buffer.create(8)
		buffer.writef64(buf_34, 0, arg_8)
		Events.alpha:_fire(buf_34)
	end
	local _arg0 = function(item)
		return (function()
			local arg_12 = item * 2
			local buf_39 = buffer.create(8)
			buffer.writef64(buf_39, 0, arg_12)
			return Events.alpha:_fire(buf_39)
		end)()
	end
	for _k, _v in items do
		_arg0(_v, _k - 1, items)
	end
	local text = `{Events.charlie:_fire()}`
	return { sent, picked, list, text }
end
local function returned(value)
	local arg_13 = value * 2
	local buf_40 = buffer.create(8)
	buffer.writef64(buf_40, 0, arg_13)
	return Events.alpha:_fire(buf_40)
end
-- A send inside another send's argument, and inside a callback.
local function nested(value)
	local arg_14 = (function()
		local arg_15 = value + 1
		local buf_42 = buffer.create(8)
		buffer.writef64(buf_42, 0, arg_15)
		Events.alpha:_fire(buf_42)
		return value
	end)()
	local buf_41 = buffer.create(8)
	buffer.writef64(buf_41, 0, arg_14)
	Events.alpha:_fire(buf_41)
	Events.lima:connect(function(text)
		return (function()
			local length_18 = #text
			local buf_43 = buffer.create(vsize(length_18) + length_18 + 1)
			local o_21 = 0
			local length_19 = #text
			o_21 = vwrite(buf_43, o_21, length_19)
			buffer.writestring(buf_43, o_21, text)
			o_21 += length_19
			buffer.writeu8(buf_43, o_21, 1)
			return Events.bravo:_fire(buf_43)
		end)()
	end)
end
-- Invoking the server and answering it.
local functions = TS.async(function(coord)
	local buf_44 = buffer.create(8)
	buffer.writef64(buf_44, 0, 1)
	local sum = TS.await(Functions.november:_invoke(buf_44))
	local arg_16 = `n{sum}`
	local length_20 = #arg_16
	local buf_45 = buffer.create(vsize(length_20) + length_20)
	local o_22 = 0
	local length_21 = #arg_16
	o_22 = vwrite(buf_45, o_22, length_21)
	buffer.writestring(buf_45, o_22, arg_16)
	o_22 += length_21
	local name = TS.await(Functions.oscar:_invokeWithTimeout(5, buf_45))
	TS.await(Functions.papa:_invoke())
	local arg_17 = {
		x = sum,
		y = #name,
	}
	local buf_46 = buffer.create(16)
	buffer.writef64(buf_46, 0, arg_17.x)
	buffer.writef64(buf_46, 8, arg_17.y)
	local back = TS.await(Functions.quebec:_invoke(buf_46))
	local target_1 = Functions.romeo
	local callback = function(value)
		return `{value}`
	end
	target_1:_setCallback(callback, function(value_10)
		local length_22 = #value_10
		local buf_47 = buffer.create(vsize(length_22) + length_22)
		local o_23 = 0
		local length_23 = #value_10
		o_23 = vwrite(buf_47, o_23, length_23)
		buffer.writestring(buf_47, o_23, value_10)
		o_23 += length_23
		return { buf_47 }
	end)
	return back or coord
end)
-- A class method: `this` is a plain read.
local Sender
do
	Sender = setmetatable({}, {
		__tostring = function()
			return "Sender"
		end,
	})
	Sender.__index = Sender
	function Sender.new(...)
		local self = setmetatable({}, Sender)
		return self:constructor(...) or self
	end
	function Sender:constructor()
		self.events = Events
	end
	function Sender:send(value)
		local buf_48 = buffer.create(8)
		buffer.writef64(buf_48, 0, value)
		self.events.alpha:_fire(buf_48)
		local arg_18 = tostring(value)
		local length_24 = #arg_18
		local buf_49 = buffer.create(vsize(length_24) + length_24 + 1)
		local o_24 = 0
		local length_25 = #arg_18
		o_24 = vwrite(buf_49, o_24, length_25)
		buffer.writestring(buf_49, o_24, arg_18)
		o_24 += length_25
		if not (value >= 0 and value <= 255 and value % 1 == 0) then
			codec.checkWidth("u8", value, "'bravo' [1]")
		end
		buffer.writeu8(buf_49, o_24, value)
		self.events.bravo:_fire(buf_49)
	end
end
local buf = buffer.create(8)
buffer.writef64(buf, 0, 1)
-- At the top of the file.
Events.alpha:_fire(buf)
Events.charlie:_fire()
local arg = compute()
local length = #"top"
local buf_1 = buffer.create(vsize(length) + length + 1)
local o = 0
local text_1 = "top"
local length_1 = #text_1
o = vwrite(buf_1, o, length_1)
buffer.writestring(buf_1, o, text_1)
o += length_1
if not (arg >= 0 and arg <= 255 and arg % 1 == 0) then
	codec.checkWidth("u8", arg, "'bravo' [1]")
end
buffer.writeu8(buf_1, o, arg)
Events.bravo:_fire(buf_1)
return {
	shapes = shapes,
	spreads = spreads,
	positions = positions,
	returned = returned,
	nested = nested,
	functions = functions,
	Events = Events,
	Functions = Functions,
	Sender = Sender,
}
