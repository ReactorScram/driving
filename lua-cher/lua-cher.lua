local ffi = require "ffi"
local lume = require "lume"

local cher = ffi.load "./libcherenkov.so"

ffi.cdef [[
typedef void * CherPtr;

typedef struct {
	int32_t x;
	int32_t y;
} PodVec2;

CherPtr cher_new (float radius, PodVec2 player_start);
void cher_add_polycapsule (CherPtr, int32_t n, PodVec2 * points);
void cher_delete (CherPtr);

void cher_step (CherPtr);
PodVec2 cher_get_player (CherPtr);
]]

local scale_den = 1.0
local polylines = require "polylines"

--table.remove (polylines, #polylines)
--table.remove (polylines, 2)
--table.remove (polylines, 1)

local function into_cher_space (pos)
	local pod = ffi.new ("PodVec2")
	
	pod.x = (pos [1] - 400) * 65536.0 / scale_den
	pod.y = (pos [2] - 300) * 65536.0 / scale_den
	
	return pod
end

local function new (pos)
	local ctx = ffi.gc (cher.cher_new (8.0 / scale_den, into_cher_space (pos)), cher.cher_delete)

	local function add_polycapsule (points)
		local pods = ffi.new ("PodVec2 [?]", #points)
		
		for i = 1, #points do
			--print (points [i][1], points [i][2])
			pods [i - 1].x = (points [i][1] - 400) * 65536.0 / scale_den
			pods [i - 1].y = (points [i][2] - 300) * 65536.0 / scale_den
		end
		
		cher.cher_add_polycapsule (ctx, #points, pods)
	end
	
	for _, polyline in ipairs (polylines) do
		add_polycapsule (polyline)
	end
	
	return ctx
end

local function get_player (ctx)
	return cher.cher_get_player (ctx)
end

local function step (ctx)
	cher.cher_step (ctx)
end

return {
	new = new,
	step = step,
	get_player = get_player,
	scale_den = scale_den,
	polylines = polylines,
}
