local Cher = require "lua-cher"
local polylines = Cher.polylines
local FixedTimestep = require "fixed-timestep"

local ctx = Cher.new ({400, 100})

local ctxs = {}
for i = 1, 50 do
	ctxs [i] = Cher.new {400 + (i - 25) * 2, 100}
end

local timestep = FixedTimestep (60, 1)
local scale_den = Cher.scale_den

local steps = 0

local playing = false

function love.draw ()
	local function draw_player (ctx)
		local player = Cher.get_player (ctx)
		
		local x = math.floor (player.x * scale_den / 65536.0) + 400
		local y = math.floor (player.y * scale_den / 65536.0) + 300
		
		--print (x, y)
		
		love.graphics.circle ("line", x, y, 8)
	end
	
	local alpha = 255
	
	local colors = {
		{255, 64, 64, alpha},
		{64, 255, 64, alpha},
		{64, 64, 255, alpha},
		{255, 255, 64, alpha},
		{255, 64, 255, alpha},
		{64, 255, 255, alpha},
	}
	
	local function get_color (i)
		return colors [(i - 1) % 6 + 1]
	end
	
	for i, ctx in ipairs (ctxs) do
		love.graphics.setColor (get_color (i))
		draw_player (ctx)
	end
	
	
	
	for i, polyline in ipairs (polylines) do
		love.graphics.setColor (get_color (i))
		
		for i = 1, #polyline - 1 do
			local j = i + 1
			local a = polyline [i]
			local b = polyline [j]
			
			love.graphics.line (a [1], a [2], b [1], b [2])
		end
		
		local function draw_circle (p)
			love.graphics.circle ("line", p [1], p [2], 3)
		end
		
		draw_circle (polyline [1])
		draw_circle (polyline [#polyline])
	end
end

function love.keypressed (key)
	if key == "space" then
		playing = not playing
	end
end

function love.update (dt)
	if playing then
		timestep:step (dt, function ()
			for _, ctx in ipairs (ctxs) do
				Cher.step (ctx)
			end
		end)
		
		steps = steps + 1
		--print (steps)
	end
end
