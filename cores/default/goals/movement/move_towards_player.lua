mtp = {}

function mtp:should_start()
	return true
end

function mtp:init() 
	self.timeout = 60
end

function mtp:update()
	self.timeout -= delta_time() * 60
	return move_towards(pos_self, pos_player, 1.2)
end

function mtp:should_stop()
	return self.timeout <= 0
end

return mtp
