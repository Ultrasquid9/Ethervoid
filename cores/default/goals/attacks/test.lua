test = {}

function test:should_start()
	if prev_goal == "default:attacks/test" then 
		return false
	end

	dist = distance_between(pos_self, pos_player)
	return round(dist) % 3 == 0
end

function test:init()
	self.pos_target = pos_player
end

function test:update(attacks, anim)
	-- table.insert(attacks, "")

	return { 
		x = 0, 
		y = 0,
	}
end

function test:should_stop()
	return false
end

return test
