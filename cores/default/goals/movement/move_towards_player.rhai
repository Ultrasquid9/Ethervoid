import "default:movement/test_mod" as test;

fn should_start() {
	print(true ? [test::TEST, "false"]);
	true
}

fn init() {
	let timeout = 120;
}

fn update() {
	timeout -= 1;
	pos_self.move_towards(pos_player, 1.0)
}

fn should_stop() {
	timeout <= 0
}
