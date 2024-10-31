use cores::map::get_maps;
use combat::{try_parry, Attack};
use draw::{draw, textures::load_texture};
use enemy::Enemy;
use entity::MovableObj;
use player::Player;
use macroquad::prelude::*;

use crate::{input::is_down, State};

mod player;
mod enemy;
mod cores;
mod draw;
mod entity;
mod combat;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The camera
	let mut camera = Vec2::new(0., 0.);

	// The player, enemies, and attacks
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	let mut attacks: Vec<Attack> = Vec::new(); // Creates a list of attacks 

	// Textures
	let texture = load_texture("./assets/textures/tiles/grass-test.png");
	
	// The maps
	let maps = get_maps(); // Creates a list of Maps
	let current_map = String::from("default:test"); // Stores the map the player is currently in

	// Returns the current map 
	let get_map = || -> Vec<Vec2> {
		return maps.get(&current_map).unwrap().points.clone();
	};

	// Populating the enemies with data from the maps
	for i in maps.get(&current_map).unwrap().enemies.clone() {
		enemies.push(Enemy::new(i.1, i.0, load_texture("./assets/textures/entity/enemies/test.png")))
	}

	loop {
		// Updates the player
		player.update(&get_map());

		// Attacking
		if is_down("Sword", &player.config) && player.swords[0].cooldown == 0 {
			player.swords[0].cooldown = 16;
			attacks.push(player.attack_sword());
		}
		if is_down("Gun", &player.config) && player.guns[0].cooldown == 0 {
			player.guns[0].cooldown = 16;
			attacks.push(player.attack_gun());
		}

		// Updates attacks
		if attacks.len() > 0 {
			for i in &mut attacks {
				i.update(&mut enemies, &mut player, &get_map());
			}

			try_parry(&mut attacks);

			attacks.retain(|x| !x.should_rm());
		}

		// Updates enemies
		if enemies.len() > 0 {
			for i in &mut enemies {
				i.update(&mut attacks, &mut player, &get_map());
			}

			enemies.retain(|x| !x.stats.should_kill());
		}

		// Updates the camera
		// TODO: Attempt to replace with .lerp()
		camera = camera.move_towards(
			player.stats.get_pos(), 
			camera.distance(player.stats.get_pos()) / 6.
		);
		// Draws the player and enemies
		draw(&mut camera, &player, &enemies, &attacks, &texture, &get_map());

		// Quits the game
		if is_down("Quit", &player.config) {
			println!("Returning to the main menu");
			return State::Menu;
		}

		next_frame().await;
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
