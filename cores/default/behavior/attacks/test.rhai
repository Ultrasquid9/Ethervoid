permanent target_pos <= player_pos;

attacks.push(
	new_physical(
		10.,
		20.,
		target_pos
	)
);

if self_pos.distance_between(target_pos) < 5. {
	end();
} else {
	self_pos.move_towards(target_pos, 1.0);
}
