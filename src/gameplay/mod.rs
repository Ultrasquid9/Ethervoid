use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

use crate::{input::get_keycode, State};

mod player;
mod enemy;

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pub pos: Vec2,
	pub health: usize
}

pub async fn gameplay() -> State {
	let mut player = Player::new(); // Creates a player
	let mut enemy = Enemy::new(); // Creates an enemy

	loop {
		// Draws the background
		clear_background(RED);

		// Creates a camera targetting the player
        set_camera(&Camera2D {
			zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
            target: player.stats.pos,
            ..Default::default()
        });

		// Updates the player and all enemies
		player.update();
		enemy.update(&mut player);

		// The Player and enemies
        draw_circle(player.stats.pos.x, player.stats.pos.y, 15.0, YELLOW); // Player
		draw_circle(enemy.stats.pos.x, enemy.stats.pos.y, 15.0, GREEN); // Test Object

		set_default_camera();
 
		// The UI
		draw_text(&format!("{}", player.stats.health), 32.0, 64.0, camera_scale() / 10., BLACK);

		// Quits the game
		if is_key_down(get_keycode(&player.config, "Quit")) {
			println!("Quitting the game");
			return State::Quit;
		}

		next_frame().await;
	}
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}