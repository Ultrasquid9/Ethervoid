use std::{borrow::BorrowMut, collections::HashMap, fs};

use macroquad::math::{vec2, Vec2};
use rhai::Engine;

use crate::gameplay::{enemy::Enemy, player::Player};

use super::get_files;

#[derive(Clone)]
pub struct AttackScript {
	current_target: Vec2,
	script: String
}

impl AttackScript {
	/// Creates an attack with the script at the provided directory
	pub fn from(dir: String) -> Self {
		AttackScript {
			current_target: Vec2::new(0., 0.),
			script: fs::read_to_string(dir).unwrap()
		}
	}

	/// Sets the position that the enemy will target
	pub fn set_target(&mut self, target: Vec2) { 
		self.current_target = target 
	}

	/// Reads the attack script. Returns true if the enemy has reached the target, or if the enemy could not move
	pub fn read_script(&mut self, enemy: &mut Enemy, player: &Player, map: &Vec<Vec2>) -> bool {
		let mut engine = Engine::new(); // Creating the Rhai engine

		// Cloning data that will be passed to getter methods
		// No, you can't do so within the method itself, I tried 
		let player_pos = player.stats.get_pos().clone();
		let enemy_pos = enemy.stats.get_pos().clone();
		let target_pos = self.current_target.clone();

		// The move_towards method didn't work for some reason so I have to make a shitty copy 
		fn move_towards(pos1: Vec2, pos2: Vec2, distance: f32) -> Vec2 {
			return pos1.move_towards(pos2, distance)
		}

		engine
			// Registerring the Vec2 and functions related to it
			.register_type_with_name::<Vec2>("position")
			.register_fn("move_towards", move_towards)

			// Getter methods for the player and enemy positions
			.register_fn("player_pos", move || player_pos)
			.register_fn("enemy_pos", move || enemy_pos)
			.register_fn("target_pos", move || target_pos)
			.register_fn("end", move || Vec2::new(999999., 999999.));

		// Executing the script
		let new_pos = match engine.eval::<Vec2>(&self.script) {
			Ok(new_pos) => new_pos,
			Err(e) => panic!("Bad script: {}", e)
		};

		// A horrible hacky way of checking if the 'end' keyword was called
		if new_pos == vec2(999999., 999999.) {
			return true;
		} else {
			enemy.stats.try_move(new_pos, map);
		}

		// Returns true if the enemy could not move or if the enemy has reached the target
		// Otherwise, returns false
		if enemy.stats.get_pos() == self.current_target
		|| enemy.stats.get_pos() != new_pos {
			return true
		} else {
			return false
		}
	}
}

/// Provides a HashMap containing all Attacks
pub fn get_attacks() -> HashMap<String, AttackScript> {
	let mut attacks: HashMap<String, AttackScript> = HashMap::new();

	for i in get_files(String::from("attacks")) {
		attacks.insert(
			name_from_filename(&i),
			AttackScript::from(i)
		);
	}

	return attacks;
}

/// Hacky temporary method to convert a directory into a name
fn name_from_filename(dir: &str) -> String {
	let split_slashes: Vec<&str> = dir.split(&['/', '\\'][..]).collect();
	let dir_no_slashes = split_slashes[split_slashes.len() - 1];

	let split_period: Vec<&str> = dir_no_slashes.split(".").collect();
	let dir_no_period = split_period[0];
	
	return dir_no_period.to_owned();
}
