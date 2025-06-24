local wander = {
	should_start = function()
		return true
	end,

	should_stop = function()
		return false
	end,
}

function wander:new_target()
	local range = 32

	self.target = {
		x = math.random(self.center.x - range, self.center.x + range),
		y = math.random(self.center.y - range, self.center.y + range),
	}
end

function wander:init(pos_self)
	self.center = pos_self
	self.cooldown = 0
	self:new_target()
end

function wander:update(pos_self)
	if self.cooldown > 0 then
		self.cooldown -= engine.delta_time()

		return pos_self
	end

	if distance_between(pos_self, self.target) < 5 then
		local old_target = self.target

		self.cooldown = math.random(1, 2)
		self:new_target()

		return old_target
	end

	return move_towards(pos_self, self.target, 2)
end

return wander
