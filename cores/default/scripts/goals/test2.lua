local test2 = {}

function test2:should_start()
	if goals.previous() == "default:attacks/test2" then
		return false
	end

	local dist = distance_between(position.self(), position.player())
	return round(dist) % 2 == 0
end

function test2:init()
	self.countdown = 20
end

function test2:update(attacks, anim)
	if self.countdown == 20. then
		anim = "toss"
	end

	self.countdown -= delta_time() * 60

	if self.countdown <= 0 then
		table.insert(
			attacks,
			attack.projectile(12, 10, position.self(), position.player(), "default:attacks/projectile-enemy")
		)
	end

	return position.self()
end

function test2:should_stop()
	return self.countdown <= 0
end

return test2
