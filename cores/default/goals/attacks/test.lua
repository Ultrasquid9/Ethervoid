test = {}

function test:should_start()
	if prev_goal == "default:attacks/test" then 
		return false
	end

	return true
end

function test:init()
	self.pos_target = pos_player
end

function test:update(attacks, anim)
	attacks[1] = "test"

	return { 
		x = 0, 
		y = 0,
	}
end

function test:should_stop()
	return false
end

return test
