local mtp = {
	should_start = function()
		return true
	end,
}

function mtp:init()
	self.timeout = 60
end

function mtp:update(pos_self, pos_player)
	self.timeout -= engine.delta_time() * 60
	return move_towards(pos_self, pos_player, 1.2)
end

function mtp:should_stop()
	return self.timeout <= 0
end

return mtp
