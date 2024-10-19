use std::collections::HashMap;

use builders::mapbuilder::{get_mapbuilders, MapBuilder};
use draw::draw;
use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

use crate::{input::get_keycode, State};

mod player;
mod enemy;
mod builders;
mod draw;
mod movement;
mod combat;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The player and enemies themselves
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	
	// The maps
	let maps = get_mapbuilders(); // Creates a list of MapBuilders
	let current_map = String::from("Test"); // Stores the map the player is currently in

	// Populating the enemies with data from the maps
	for i in maps.get(&current_map).unwrap().enemies.clone() {
		enemies.push(Enemy::from_builder(i.1, i.0))
	}

	loop {
		// Updates the player and all enemies
		player.update(&get_map(&maps, &current_map));

		if enemies.len() > 0 {
			for i in &mut enemies {
				i.update(&mut player, &get_map(&maps, &current_map));
		
				if is_key_down(get_keycode(&player.config, "Attack")) {
					if i.stats.get_pos().distance(player.stats.get_pos()) < 64.0 {
						i.damage(1);
					}
				}
			}

			let enemies_to_kill = enemies_to_kill(&enemies);
			enemies.retain(|_| *enemies_to_kill.iter().next().unwrap());
		}

		// Draws the player and enemies
		draw(&player, &enemies, &get_map(&maps, &current_map));

		// Quits the game
		if is_key_down(get_keycode(&player.config, "Quit")) {
			println!("Quitting the game");
			return State::Quit;
		}

		next_frame().await;
	}
}

/// Gets the enemies that should be retained
fn enemies_to_kill(enemies: &Vec<Enemy>) -> Vec<bool> {
	let mut enemies_to_kill: Vec<bool> = Vec::new();

	for i in enemies {
		enemies_to_kill.push(!i.should_kill());
	}

	return enemies_to_kill;
}

/// Gets the map at the provided String
fn get_map(maps: &HashMap<String, MapBuilder>, current_map: &str) -> Vec<Vec2> {
	return maps.get(current_map).unwrap().points.clone();
}

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y)
}