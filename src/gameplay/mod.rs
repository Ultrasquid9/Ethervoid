use player::Player;
use macroquad::prelude::*;

mod player;

pub async fn gameplay() {
	let mut player = Player::new();

	loop {
		clear_background(RED);

		player.update();
        draw_circle(player.pos.x, player.pos.y, 15.0, YELLOW);

		next_frame().await;
	}
}