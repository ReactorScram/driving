local ffi = require "ffi"

local cher = ffi.load "target/debug/libcherenkov.so"

ffi.cdef [[
int32_t cher_add (int32_t a, int32_t b);

typedef void * CherCtx;

CherCtx cher_new (float radius);
void cher_delete (CherCtx);
]]

print ("3 + 4 =")
print (cher.cher_add (3, 4))

local ctx = ffi.gc (cher.cher_new (5.0), cher.cher_delete)

print ("Context: ", ctx)
