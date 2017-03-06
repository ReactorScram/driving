local Object = require "classic"
local M = Object:extend ()

local FixedTimestep = require "fixed-timestep"
local loadParticles = require "load-obj"

function M:new (love)
	self.love = love
	self.radius = 20
	
	self.time = 1
	
	local f = io.open ("lines.obj", "rb")
	self.particles = loadParticles (f)
	f.close ()
end

function M:draw ()
	self.love.graphics.setColor (200, 200, 200, 255)
	
	for _, particle_history in ipairs (self.particles) do
		local particle = particle_history [self.time]
		local alive = true
		self.love.graphics.setColor (200, 200, 200, 255)
		if not particle then
			particle = particle_history [#self.time]
			alive = false
			self.love.graphics.setColor (0, 88, 151, 255)
		end
		
		self.love.graphics.circle ("line", particle [1] * 256.0 + (800 - 512) / 2, particle [2] * 256.0, self.radius, 19)
	end
end

return M
