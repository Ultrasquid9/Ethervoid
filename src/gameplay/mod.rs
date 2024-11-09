use cores::map::{get_maps, Map};
use combat::{try_parry, Attack};
use draw::{access_texture, clean_textures, create_textures, draw};
use enemy::Enemy;
use entity::MovableObj;
use player::Player;
use macroquad::prelude::*;

use crate::State;

mod combat;
mod cores;
mod doors;
mod draw;
mod enemy;
mod entity;
mod player;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The camera
	let mut camera = Vec2::new(0., 0.);

	// Textures
	// NOTE: populates a static HashMap. Ensure you call the `clean_attack_textures` function when quitting the game. 
	create_textures();

	// The player, enemies, and attacks
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	let mut attacks: Vec<Attack> = Vec::new(); // Creates a list of attacks 
	
	// The maps
	let maps = get_maps(); // Creates a list of Maps
	let mut current_map = String::from("default:test"); // Stores the map the player is currently in

	// Populating the enemies with data from the maps
	populate_enemies(&mut enemies, maps.get(&current_map).unwrap());

	loop {
		// Updates the player
		player.update(&mut camera, &mut enemies, &mut attacks, &mut current_map, &maps);

		// Attacking
		if player.config.keymap.sword.is_down() && player.swords[0].cooldown == 0 {
			player.swords[0].cooldown = 16;
			attacks.push(player.attack_sword());
		}
		if player.config.keymap.gun.is_down() && player.guns[0].cooldown == 0 {
			player.guns[0].cooldown = 16;
			attacks.push(player.attack_gun());
		}

		// Updates attacks
		if attacks.len() > 0 {
			for i in &mut attacks {
				i.update(&mut enemies, &mut player, &maps.get(&current_map).unwrap());
			}

			try_parry(&mut attacks);

			attacks.retain(|x| !x.should_rm());
		}

		// Updates enemies
		if enemies.len() > 0 {
			for i in &mut enemies {
				i.update(&mut attacks, &mut player, &maps.get(&current_map).unwrap());
			}

			enemies.sort();

			enemies.retain(|x| !x.stats.should_kill());
		}

		// Updates the camera
		// TODO: Attempt to replace with .lerp()
		camera = camera.move_towards(
			player.stats.get_pos(), 
			camera.distance(player.stats.get_pos()) / 6.
		);

		// Draws the player and enemies
		draw(
			&mut camera, 
			&player, 
			&enemies, 
			&attacks, 
			&maps.get(&current_map).unwrap()
		).await;

		// Quits the game
		if player.config.keymap.quit.is_pressed() {
			clean_textures();

			println!("Returning to the main menu");
			return State::Menu;
		}

		next_frame().await;
	}
}

pub fn populate_enemies(enemies: &mut Vec<Enemy>, map: &Map) {
	enemies.clear();

	for i in map.enemies.clone() {
		enemies.push(Enemy::new(i.1, i.0, access_texture("default:entity/player/player_spritesheet_wip")))
	}
}

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y);
}

/// Converts the inputted tuple of f32 into a Vec2
pub fn tuple_to_vec2(tup: (f32, f32)) -> Vec2 {
	return Vec2::new(tup.0, tup.1);
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> Vec2 {
	tuple_to_vec2(mouse_position()) - Vec2::new(screen_width() / 2., screen_height() / 2.)
}

/// Gets the delta time
pub fn get_delta_time() -> f32 {
	get_frame_time() * 100. * (2./3.)
}
