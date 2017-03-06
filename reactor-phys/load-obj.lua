-- Loads a particle recording from the .obj files
-- that cherenkov puts out

local function load (reader)
	local particles = {}
	local current_particle = nil
	local vertex_count = 0
	
	for line in reader:lines () do
		if line:sub (1, 2) == "v " then
			if not current_particle then
				current_particle = {}
			end
			
			local sx, sy = line:match "v (%S+) 0 (%S+)"
			local x = tonumber (sx)
			local y = tonumber (sy)
			
			table.insert (current_particle, {x, y})
		else
			if current_particle then
				--print (#current_particle .. " frames")
				
				table.insert (particles, current_particle)
				current_particle = nil
			end
		end
	end
	
	--print (#particles .. " particles")
	
	return particles
end

return load
