use std::fs;

use macroquad::prelude::*;
use textures::{draw_tilemap, pixel_offset, render_texture};

use super::{combat::Attack, enemy::Enemy, entity::MovableObj, player::Player};

mod textures;

const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

/// Draws the content of the game
pub fn draw(camera: &mut Vec2, player: &Player, enemies: &Vec<Enemy>, attacks: &Vec<Attack>, map: &Vec<Vec2>) {
	clear_background(RED); // Draws the background

	let camera = Vec2::new(
		pixel_offset(camera.x),
		pixel_offset(camera.y),
	);

	// Creates a camera targetting the player
	set_camera(&Camera2D {
		zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
		target: camera,
		..Default::default()
	});
	draw_tilemap();

	// Draws the map
	for i in 0..map.len() {
		match map.get(i + 1) {
			Some(_) => draw_line(
				map.get(i).unwrap().x, 
				map.get(i).unwrap().y, 
				map.get(i + 1).unwrap().x, 
				map.get(i + 1).unwrap().y, 
				4., 
				BLUE
			),
			None => draw_line(
				map.get(i).unwrap().x, 
				map.get(i).unwrap().y, 
				map.get(0).unwrap().x, 
				map.get(0).unwrap().y, 
				4., 
				BLUE
			),
		}
	}

	// Drawing the Player, enemies, and attacks
	if attacks.len() > 0 {
		for i in attacks {
			if i.is_hitscan() {
				draw_line(
					i.pos.x, 
					i.pos.y, 
					i.get_target().x, 
					i.get_target().y, 
					6., 
					PURPLE
				); 
			} else {
				draw_circle(i.pos.x, i.pos.y, i.size, PURPLE); 
			}
		}
	}

	// The player
	render_texture(
		&load_pic("".to_string()), 
		player.stats.get_pos()
	);

	if enemies.len() > 0 {
		for i in enemies {
			draw_circle(i.stats.x(), i.stats.y(), i.stats.size as f32, GREEN); // Enemies
		}
	}

	set_default_camera();
 
	// Drawing a temporary UI
	draw_text(&format!("{}", player.stats.get_health()), 32.0, 64.0, camera_scale() / 10., BLACK);
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}

/// Loads a picture from the provided directory (NOTE: WIP)
fn load_pic(_dir: String) -> Texture2D {
	let texture = Texture2D::from_file_with_format(
		fs::read("./assets/textures/entity/player/player-indev.png").unwrap().as_slice(), 
		None
	);
	texture.set_filter(FilterMode::Nearest);
	return texture;
}
