permanent countdown <- 20.;

if countdown == 20. {
	self_current_anim = "toss";
} else if countdown > 0. {
	countdown -= delta();
	return self_pos;
}

attacks.push(
	new_projectile(
		12.,
		self_pos,
		player_pos,
		"default:attacks/projectile-enemy"
	)
);
end()
