local Cher = require "lua-cher"
local polylines = require "polylines"
local FixedTimestep = require "fixed-timestep"

local ctx = Cher.new ()
local timestep = FixedTimestep (10, 1)
local scale_den = Cher.scale_den

function love.draw ()
	local player = Cher.get_player (ctx)
	
	love.graphics.circle ("line", player.x * scale_den / 65536.0 + 400, player.y * scale_den / 65536.0 + 300, 5)
	
	for _, polyline in ipairs (polylines) do
		for i = 1, #polyline - 1 do
			local j = i + 1
			local a = polyline [i]
			local b = polyline [j]
			
			love.graphics.line (a [1], a [2], b [1], b [2])
		end
	end
end
--[[
function love.keypressed (key)
	if key == "space" then
		Cher.step (ctx)
	end
end
--]]

function love.update (dt)
	timestep:step (dt, function ()
		Cher.step (ctx)
	end)
end
