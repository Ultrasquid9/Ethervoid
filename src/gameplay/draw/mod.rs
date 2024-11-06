use downscale::{downscale, to_texture};
use futures::{future::join_all, join};
use imageproc::image::DynamicImage;
use macroquad::prelude::*;
use textures::{draw_tilemap, pixel_offset, render_texture};

use crate::gameplay::entity::MovableObj;

use super::{combat::Attack, enemy::Enemy, player::Player};

pub mod textures;
pub mod texturedobj;
pub mod downscale;

const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

/// Draws the content of the game
pub async fn draw(camera: &mut Vec2, player: &Player, enemies: &Vec<Enemy<'_>>, attacks: &Vec<Attack>, textures: &Vec<DynamicImage>, map: &Vec<Vec2>) {
	// Draws the background
	clear_background(Color::from_rgba(
		46, 
		34, 
		47, 
		255
	)); 

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

	// Futures containing operations to perform
	let mut attack_futures = Vec::new();
	let mut entity_futures = Vec::new();

	// Tilemap
	draw_tilemap(to_texture(textures[0].clone())).await;

	// Appl 
	render_texture(
		&downscale(&textures[1], 16, 45.), 
		Vec2::new(200., 200.), 
		None
	).await;

	// The map
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

	// Attacks
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
				attack_futures.push(i.texture.render());
			}
		}
	}

	// Enemies and the player
	if enemies.len() > 0 {
		// This reminds me of that one time I went to Italy,
		// because this is a giant mass of spaghetti 

		for i in enemies.iter().enumerate() {
			if i.0 == enemies.len() - 1 {
				if player.stats.get_pos().y >= i.1.stats.get_pos().y {
					entity_futures.push(i.1.stats.texture.render());
					entity_futures.push(player.stats.texture.render());
				} else {
					entity_futures.push(player.stats.texture.render());
					entity_futures.push(i.1.stats.texture.render());
				}

				break
			}

			if enemies[i.0 + 1].stats.get_pos().y >= player.stats.get_pos().y
			&& i.1.stats.get_pos().y >= player.stats.get_pos().y {
				entity_futures.push(player.stats.texture.render());
			}

			entity_futures.push(i.1.stats.texture.render());
		}
	} else {
		entity_futures.push(player.stats.texture.render());
	}

	// Performing operations pushed to the futures
	join!(
		join_all(entity_futures),
		join_all(attack_futures)
	);

	// Returning to the default camera, any future textures are UI-based
	set_default_camera();
 
	// Drawing a temporary UI
	draw_text(&format!("{}", player.stats.get_health()), 32.0, 64.0, camera_scale() / 10., BLACK);
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}
