use std::{collections::HashMap, fs};

use macroquad::math::Vec2;
use rhai::Engine;

use crate::gameplay::{enemy::Enemy, player::Player};

use super::get_files;

#[derive(Clone)]
pub struct Attack {
	current_target: Vec2,
	script: String
}

impl Attack {
	pub fn from(dir: String) -> Self {
		Attack {
			current_target: Vec2::new(0., 0.),
			script: fs::read_to_string(dir).unwrap()
		}
	}

	pub fn read_script(&mut self, enemy: &mut Enemy, player: &Player, map: &Vec<Vec2>) -> bool {
		let mut engine = Engine::new(); // Creating the Rhai engine

		let player_pos = player.stats.get_pos().clone();
		let enemy_pos = enemy.stats.get_pos().clone();
		let current_terget = self.current_target.clone();

		engine
			.register_type::<Vec2>()

			.register_fn("player_pos", move || player_pos)
			.register_fn("enemy_pos", move || enemy_pos)
			.register_fn("get_target", move || current_terget);

		let new_pos = match engine.eval::<Vec2>(&self.script) {
			Ok(new_pos) => new_pos,
			Err(_) => panic!("Bad script")
		};

		enemy.stats.try_move(new_pos, map);

		if enemy.stats.get_pos() == self.current_target
		|| enemy.stats.get_pos() != new_pos {
			return true
		} else {
			return false
		}
	}
}

/// Provides a HashMap containing all Attacks
pub fn get_attacks() -> HashMap<String, Attack> {
	let mut attacks: HashMap<String, Attack> = HashMap::new();

	for i in get_files(String::from("attacks")) {
		attacks.insert(
			name_from_filename(&i),
			Attack::from(i)
		);
	}

	return attacks;
}

/// Hacky temporary method to convert a directory into a name
fn name_from_filename(dir: &str) -> String {
	let split_slashes: Vec<&str> = dir.split("/").collect();
	let dir_no_slashes = split_slashes[split_slashes.len() - 1];

	let split_period: Vec<&str> = dir_no_slashes.split(".").collect();
	let dir_no_period = split_period[0];
	
	return dir_no_period.to_owned();
}
