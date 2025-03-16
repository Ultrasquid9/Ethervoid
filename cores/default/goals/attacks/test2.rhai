fn should_start() {
	if prev_goal == "default:attacks/test2" {
		return false
	}

	let dist = pos_self.distance_between(pos_player);
	dist.round() % 2 == 0
}

fn init() {
	let countdown = 20.;
}

fn update() {
	if countdown == 20. {
		current_anim = "toss"
	}
	countdown -= delta();

	if countdown <= 0. {
		attacks.push(
			new_projectile(
				12.,
				pos_self,
				pos_player,
				"default:attacks/projectile-enemy"
			)
		);
	}

	pos_self
}

fn should_stop() {
	countdown <= 0.
}
