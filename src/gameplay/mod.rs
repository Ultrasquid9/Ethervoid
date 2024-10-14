use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

mod player;
mod enemy;

pub struct Entity {
	pub pos: Vec2,
	pub health: usize
}

pub async fn gameplay() {
	let mut player = Player::new();
	let mut enemy = Enemy::new();

	loop {
		clear_background(RED);

        set_camera(&Camera2D {
			zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
            target: player.stats.pos,
            ..Default::default()
        });

		player.update();
		enemy.update(&mut player);

        draw_circle(player.stats.pos.x, player.stats.pos.y, 15.0, YELLOW); // Player
		draw_circle(enemy.stats.pos.x, enemy.stats.pos.y, 15.0, GREEN); // Test Object

		set_default_camera();
 
		draw_text(&format!("{}", player.stats.health), 32.0, 64.0, camera_scale() / 10., BLACK); // Test UI

		next_frame().await;
	}
}

fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}