use std::collections::HashMap;

use builders::mapbuilder::{get_mapbuilders, MapBuilder};
use combat::Attack;
use draw::draw;
use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

use crate::{input::get_keycode, State};

mod player;
mod enemy;
mod builders;
mod draw;
mod entity;
mod combat;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The player, enemies, and attacks
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	let mut attacks: Vec<Attack> = Vec::new(); // Creates a list of attacks 
	
	// The maps
	let maps = get_mapbuilders(); // Creates a list of MapBuilders
	let current_map = String::from("Test"); // Stores the map the player is currently in

	// Populating the enemies with data from the maps
	for i in maps.get(&current_map).unwrap().enemies.clone() {
		enemies.push(Enemy::from_builder(i.1, i.0))
	}

	loop {
		// Updates the player
		player.update(&get_map(&maps, &current_map));

		// Attacking
		if is_key_down(get_keycode(&player.config, "Sword")) {
			attacks.push(Attack::new_physical(player.stats.get_pos(), 1, 30.));
		}

		// Updates enemies
		if attacks.len() > 0 {
			for i in &mut attacks {
				i.update(&mut enemies, &mut player);
			}

			attacks.retain(|x| !x.should_rm());
		}

		// Updates enemies
		if enemies.len() > 0 {
			for i in &mut enemies {
				i.update(&mut player, &get_map(&maps, &current_map));
			}

			enemies.retain(|x| !x.stats.should_kill());
		}

		// Draws the player and enemies
		draw(&player, &enemies, &attacks, &get_map(&maps, &current_map));

		// Quits the game
		if is_key_down(get_keycode(&player.config, "Quit")) {
			println!("Quitting the game");
			return State::Quit;
		}

		next_frame().await;
	}
}

/// Gets the map at the provided String
fn get_map(maps: &HashMap<String, MapBuilder>, current_map: &str) -> Vec<Vec2> {
	return maps.get(current_map).unwrap().points.clone();
}

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y)
}