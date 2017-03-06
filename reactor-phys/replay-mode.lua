local Object = require "classic"
local M = Object:extend ()

local FixedTimestep = require "fixed-timestep"
local loadParticles = require "load-obj"

function M:new (love)
	self.love = love
	self.radius = 20
	
	self.time = 10
	self.timestep = FixedTimestep (60, 1)
	
	local f = io.open ("lines.obj", "rb")
	self.particles = loadParticles (f)
	f.close ()
end

local function to_screen (x, y)
	return x * 256.0 + (800 - 512) / 2 + 0.5, y * 256.0
end

local function to_screen_p (p)
	return { to_screen (p [1], p [2]) }
end

local function obstacle_to_screen_p (p)
	return to_screen_p {(p [1] - 200) / 256, p [2] / 256}
end

function M:draw_line (start, stop)
	self.love.graphics.line (start [1], start [2], stop [1], stop [2])
end

function M:draw ()
	self.love.graphics.setColor (200, 200, 200, 255)
	
	local function line (t)
		for i = 2, #t do
			self:draw_line (obstacle_to_screen_p (t [i - 1]), obstacle_to_screen_p (t [i]))
		end
	end
	
	line {
		{210, 240}, 
		{210, 340},
	}
	
	line {
		{245, 240},
		{255, 340},
		{285, 340},
		{295, 240},
		{400, 340},
		{450, 240},
		{500, 340},
		{600, 350},
		{650, 330},
	}
	
	for _, particle_history in ipairs (self.particles) do
		local particle = particle_history [math.floor (self.time)]
		local next_particle = particle_history [math.floor (self.time) + 1]
		local alive = true
		self.love.graphics.setColor (200, 200, 200, 255)
		
		local interp_particle
		
		if not next_particle then
			interp_particle = particle_history [#particle_history]
			alive = false
			self.love.graphics.setColor (0, 88, 151, 255)
		else
			local t = self.time - math.floor (self.time)
			
			interp_particle = {
				particle [1] * (1 - t) + next_particle [1] * t,
				particle [2] * (1 - t) + next_particle [2] * t,
			}
		end
		
		local screen_p = to_screen_p (interp_particle)
		self.love.graphics.circle ("line", screen_p [1], screen_p [2], self.radius, 19)
	end
end

function M:keypressed (key)
	if key == "p" then
		self.paused = not self.paused
	elseif key == "i" then
		if self.paused then
			self.time = math.floor (self.time) + 1
		end
		self.paused = true
	end
end

function M:update (dt)
	local dt = dt
	if self.love.keyboard.isDown "f" then
 		dt = dt * 4
		self.paused = false
	end
	
	local dt2 = 1 / 16
	
	if self.love.keyboard.isDown "r" then
		dt2 = -dt2
		self.paused = false
	end
	
	if not self.paused then
		self.timestep:step (dt, function ()
			self.time = self.time + dt2
		end)
	end
	
	local delta = self.love.timer.getDelta ()
	if delta > 1.0 / 40.0 then
		print ("High delta", delta)
	end
end

return M
