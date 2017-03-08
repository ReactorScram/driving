local Object = require "classic"
local M = Object:extend ()

local FixedTimestep = require "fixed-timestep"
local loadParticles = require "load-obj"

function M:new (love)
	self.love = love
	self.radius = 20
	
	self.timestep = FixedTimestep (60, 1)
	self.selected_particle = 0
	
	self:reset_time ()
	self:load_data ()
end

function M:reset_time () 
	self.time = 10
end

function M:load_data ()
	local f = io.open ("lines.obj", "rb")
	self.particles = loadParticles (f)
	f.close ()
end

local function to_screen (x, y)
	return x + (800 - 512) / 2 + 0.5, y
end

local function to_screen_p (p)
	return { to_screen (p [1], p [2]) }
end

local function obstacle_to_screen_p (p)
	return to_screen_p {(p [1] - 200), p [2]}
end

function M:draw_line (start, stop)
	self.love.graphics.line (start [1], start [2], stop [1], stop [2])
end

local function get_interp_particle (particle_history, time)
	local particle = particle_history [math.floor (time)]
	local next_particle = particle_history [math.floor (time) + 1]
	local alive = true
	
	local interp_particle
	
	if not next_particle then
		interp_particle = particle_history [#particle_history]
		alive = false
	else
		local t = time - math.floor (time)
		
		interp_particle = {
			particle [1] * (1 - t) + next_particle [1] * t,
			particle [2] * (1 - t) + next_particle [2] * t,
		}
	end
	
	return interp_particle
end

local function binary_search_particle (particle_history, time)
	local lower = 1
	local upper = #particle_history
	
	if time <= particle_history [lower][3] then
		return particle_history [lower]
	elseif time >= particle_history [upper][3] then
		return particle_history [upper]
	else
		while true do
			local mid = math.floor ((lower + upper) / 2)
			
			local particle = particle_history [mid]
			local test_time = particle [3]
			if test_time < time then
				lower = mid
			elseif test_time > time then
				upper = mid
			else
				return particle
			end
			
			if upper - lower == 1 then
				local lower_particle = particle_history [lower]
				local upper_particle = particle_history [upper]
				local t = (time - lower_particle [3]) / (upper_particle [3] - lower_particle [3])
				
				return {
					lower_particle [1] * (1 - t) + upper_particle [1] * t,
					lower_particle [2] * (1 - t) + upper_particle [2] * t,
					time
				}
			end
		end
	end
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
	
	local selected_particle = self.selected_particle
	for i, particle_history in ipairs (self.particles) do
		if i == selected_particle or selected_particle == 0 then
			self.love.graphics.setColor (200, 200, 200, 64)
			
			if selected_particle == i then
			for _, keyframe in ipairs (particle_history) do
				local diff = math.abs (keyframe [3] - self.time)
				if diff <= 2.0 then
					self.love.graphics.setColor (200, 200, 200, math.min (64, 128 - 64 * diff))
					local screen_p = to_screen_p (keyframe)
			self.love.graphics.circle ("line", screen_p [1], screen_p [2], self.radius, 19)
				end
			end
			end
			
			local interp_particle = binary_search_particle (particle_history, self.time)
			
			self.love.graphics.setColor (80, 255, 0, 255)
			
			local screen_p = to_screen_p (interp_particle)
			self.love.graphics.circle ("line", screen_p [1], screen_p [2], self.radius, 19)
		end
	end
end

function M:keypressed (key)
	if key == "p" then
		--self.paused = not self.paused
	elseif key == "up" then
		self.time = self.time - 1 / 32
		self.paused = true
	elseif key == "down" then
		self.time = self.time + 1 / 32
		self.paused = true
	elseif key == "left" then
		self.selected_particle = (self.selected_particle - 1) % (#self.particles + 1)
		print ("Selected " .. self.selected_particle)
	elseif key == "right" then
		self.selected_particle = (self.selected_particle + 1) % (#self.particles + 1)
		print ("Selected " .. self.selected_particle)
	elseif key == "0" then
		self:reset_time ()
	elseif key == "f5" then
		self:load_data ()
		print "Reloaded data"
	end
end

function M:update (dt)
	local paused = true
	
	if self.love.keyboard.isDown "p" then
		paused = false
	end
	
	local dt = dt
	if self.love.keyboard.isDown "f" then
 		dt = dt * 8
		paused = false
	end
	
	local dt2 = 1 / 16
	
	if self.love.keyboard.isDown "r" then
		dt2 = -dt2
		paused = false
	end
	
	if not paused then
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
