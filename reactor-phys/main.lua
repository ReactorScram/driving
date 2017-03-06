local ReplayMode = require "replay-mode"

local mode

function love.load (arg)
	--mode = LineStripDebugMode (love)
	mode = ReplayMode (love)
	
	if arg[#arg] == "-debug" then 
		require("mobdebug").start() 
	end
end

local passedFunctions = {
	"draw",
	"keypressed",
	"update",
	"mousemoved",
	"mousepressed",
	"mousereleased",
}

for _, fnName in ipairs (passedFunctions) do
	love [fnName] = function (...)
		if mode [fnName] then
			mode [fnName] (mode, ...)
		end
	end
end
