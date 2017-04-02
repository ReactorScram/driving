local ffi = require "ffi"

local cher = ffi.load "target/debug/libcherenkov.so"

ffi.cdef [[
typedef void * CherPtr;

typedef struct {
	int32_t x;
	int32_t y;
} PlayerFrame;

CherPtr cher_new (float radius);
void cher_step (CherPtr);
PlayerFrame cher_get_player (CherPtr);
void cher_delete (CherPtr);
]]

local ctx = ffi.gc (cher.cher_new (5.0), cher.cher_delete)

print ("Context: ", ctx)

for i = 1, 10 do
	cher.cher_step (ctx)
	
	local player_frame = cher.cher_get_player (ctx)
	print (player_frame.x, player_frame.y)
end
