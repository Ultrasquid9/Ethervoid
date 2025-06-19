local test2 = {}

function test2:should_start()
	if goals.previous() == "default:goals/test2" then
		return false
	end

	local dist = distance_between(position.self(), position.player())
	return math.round(dist) % 2 == 0
end

function test2:init()
	self.countdown = 20
end

function test2:update(anim)
	if self.countdown == 20. then
		anim = "toss"
	end

	self.countdown -= engine.delta_time() * 60

	if self.countdown <= 0 then
		attack.spawn(attack.projectile(12, 10, position.self(), position.player(), "default:attacks/projectile-enemy"))
	end

	return position.self()
end

function test2:should_stop()
	if self.countdown <= 0 then
		engine.play_sound("default:sfx/sword_1")
	end

	return self.countdown <= 0
end

return test2
