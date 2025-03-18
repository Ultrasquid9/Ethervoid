fn should_start() {
	if prev_goal == "default:attacks/test" {
		return false
	}

	let dist = pos_self.distance_between(pos_player);
	dist.round() % 3 == 0
}

fn init() {
	let pos_target = pos_player;
}

fn update() {
	attacks.push(
		attack_physical(
			10.,
			20.,
			pos_self,
			pos_target,
			"default:attacks/dash"
		)
	);
	pos_self.move_towards(pos_target, 1.8)
}

fn should_stop() {
	pos_self.distance_between(pos_target) < 5.
}
