local test = {}

function test:should_start()
	if goals.previous() == "default:attacks/test" then
		return false
	end

	use("default:misc/module_test")()

	local dist = distance_between(position.self(), position.player())
	return math.round(dist) % 3 == 0
end

function test:init()
	self.target = position.player()
end

function test:update(attacks, anim)
	table.insert(attacks, attack.physical(10, 20, position.self(), self.target, "default:attacks/dash"))

	return move_towards(position.self(), self.target, 1.8)
end

function test:should_stop()
	return distance_between(position.self(), self.target) < 5
end

return test
