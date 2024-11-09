use std::collections::HashMap;

use ahash::RandomState;
use futures::{future::join_all, join};
use imageproc::image::DynamicImage;
use macroquad::prelude::*;
use once_cell::sync::Lazy;
use textures::{downscale, draw_tilemap, pixel_offset, render_texture, to_texture};

use crate::gameplay::entity::MovableObj;

use super::{combat::Attack, cores::{map::Map, textures::get_textures}, enemy::Enemy, npc::NPC, player::Player};

pub mod textures;
pub mod texturedobj;

// HashMap containing all the textures in the game 
// Everyone always says "don't do this" so fuck you I did
pub static mut TEXTURES: Lazy<HashMap<String, DynamicImage, RandomState>> = Lazy::new(|| HashMap::default());

const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

/// Draws the content of the game
pub async fn draw(camera: &mut Vec2, player: &Player, enemies: &Vec<Enemy<'_>>, npcs: &Vec<NPC>, attacks: &Vec<Attack>, map: &Map) {
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
	draw_tilemap(access_texture("default:tiles/grass_test")).await;

	// Appl 
	render_texture(
		&to_texture(downscale(access_image("default:appl"), 16)), 
		Vec2::new(200., 200.), 
		None
	).await;

	// The map
	for i in &map.doors {
		let barrier = i.to_barrier();

		draw_line(
			barrier.positions.0.0, 
			barrier.positions.0.1, 
			barrier.positions.1.0, 
			barrier.positions.1.1, 
			4., 
			RED
		);
	}

	let map = map.points.clone();
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

	// NPCs. WIP!
	for i in npcs {
		entity_futures.push(i.texture.render())
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

/// Populates the texture HashMap
/// NOTE: Please ensure you call `clean_attack_textures()` when quitting the game.
pub fn create_textures() {
	let textures = get_textures();

	unsafe {
		for i in textures {
			TEXTURES.insert(i.0, i.1);
		}
	}
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	unsafe {
		TEXTURES.get(key).unwrap().clone()
	}
}

/// Gets the texture at the provided key
pub fn access_texture(key: &str) -> Texture2D {
	to_texture(access_image(key))
}

/// Clears the texture HashMap
pub fn clean_textures() {
	unsafe { TEXTURES.clear() }
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}
