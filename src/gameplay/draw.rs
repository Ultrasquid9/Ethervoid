use macroquad::prelude::*;

use super::{enemy::Enemy, player::Player};

/// Draws the content of the game
pub fn draw(player: &Player, enemies: &Vec<Enemy>) {
	clear_background(RED); // Draws the background

	// Creates a camera targetting the player
	set_camera(&Camera2D {
		zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
		target: player.stats.get_pos(),
		..Default::default()
	});

	// Drawing the Player and enemies
    draw_circle(player.stats.x(), player.stats.y(), player.stats.size as f32, YELLOW); // Player
	if enemies.len() > 0 {
		for i in enemies {
			draw_circle(i.stats.x(), i.stats.y(), i.stats.size as f32, GREEN); // Enemies
		}
	}

	set_default_camera();
 
	// Drawing a temporary UI
	draw_text(&format!("{}", player.stats.health), 32.0, 64.0, camera_scale() / 10., BLACK);
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}
