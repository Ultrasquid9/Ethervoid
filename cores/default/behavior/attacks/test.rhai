permanent target_pos <- player_pos;

let target = target_pos;
target.x -= self_pos.distance_between(target_pos) / 1.5;

attacks.push(
	new_physical(
		10.,
		20.,
		self_pos,
		target,
		"default:attacks/dash"
	)
);

if self_pos.distance_between(target) < 5. {
	end();
} else {
	self_pos.move_towards(target, 1.8);
}
