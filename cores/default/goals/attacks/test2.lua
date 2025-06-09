test2 = {}

function test2:should_start()
	if prev_goal == "default:attacks/test2" then 
		return false
	end

	dist = distance_between(pos_self, pos_player)
	return round(dist) % 2 == 0
end

function test2:init()
	self.countdown = 20
end

function test2:update(attacks, anim)
	if self.countdown == 20. then
		anim = "toss"
	end

	self.countdown -= delta_time()

	if self.countdown <= 0 then
		table.insert(attacks, attack.projectile(
			12,
			10,
			pos_self,
			self.pos_target,
			"default:attacks/projectile-enemy"
		))
	end

	return pos_self
end

function test2:should_stop()
	return self.countdown <= 0
end

return test2
