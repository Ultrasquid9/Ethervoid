local test = {}

function test:should_start(pos_self, pos_player, prev_goal)
	if prev_goal == "default:goals/test" then
		return false
	end

	-- Test of both modules and logging
	-- Theoretically, this code should never run, but it *is* valid
	if use("default:misc/module_test")() == pos_player then
		log.info(pos_player)
	end

	local dist = distance_between(pos_self, pos_player)
	return math.round(dist) % 3 == 0
end

function test:init(_, pos_player)
	self.target = pos_player
end

function test:update(pos_self, _, anim)
	attack.spawn(attack.physical(10, 20, pos_self, self.target, "default:attacks/dash"))

	return move_towards(pos_self, self.target, 1.8)
end

function test:should_stop(pos_self, _)
	return distance_between(pos_self, self.target) < 5
end

return test
