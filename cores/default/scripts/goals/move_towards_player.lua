local mtp = {
	should_start = function()
		return true
	end,
}

function mtp:init()
	self.timeout = 60
end

function mtp:update()
	self.timeout -= engine.delta_time() * 60
	return move_towards(position.self(), position.player(), 1.2)
end

function mtp:should_stop()
	return self.timeout <= 0
end

return mtp
