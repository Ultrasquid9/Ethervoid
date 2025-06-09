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
	table.insert(attacks, attack.physical(
		10,
		20,
		pos_self,
		self.pos_target,
		"default:attacks/dash"
	))

	return move_towards(pos_self, self.pos_target, 1.8)
end

function test:should_stop()
	return distance_between(pos_self, self.pos_target) < 5
end

return test
