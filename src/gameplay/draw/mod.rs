use downscale::{downscale, to_texture};
use imageproc::image::DynamicImage;
use macroquad::prelude::*;
use textures::{draw_tilemap, pixel_offset, render_texture};

use super::{combat::{Attack, Owner}, enemy::Enemy, player::Player};

pub mod textures;
pub mod texturedentity;
pub mod downscale;

const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

/// Draws the content of the game
pub fn draw(camera: &mut Vec2, player: &Player, enemies: &Vec<Enemy>, attacks: &Vec<Attack>, textures: &Vec<DynamicImage>, map: &Vec<Vec2>) {
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
	draw_tilemap(&to_texture(textures[0].clone()));

	// Appl 
	render_texture(
		&downscale(&textures[1], 16, 45.), 
		Vec2::new(200., 200.), 
		None
	);

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
				draw_circle(i.pos.x, i.pos.y, i.size, if i.owner == Owner::Player {
					PURPLE
				} else {
					ORANGE
				}); 
			}
		}
	}

	// The player
	player.stats.texture.render();

	// Enemies
	if enemies.len() > 0 {
		for i in enemies {
			i.stats.texture.render();
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
