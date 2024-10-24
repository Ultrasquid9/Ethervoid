use std::{collections::HashMap, fs};

use macroquad::math::{vec2, Vec2};
use rhai::{Dynamic, Engine, Scope};

use crate::gameplay::{combat::{Attack, Owner}, enemy::Enemy, player::Player};

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
	pub fn read_script(&mut self, enemy: &mut Enemy, player: &Player, map: &Vec<Vec2>, attacks: &mut Vec<Attack>) -> bool {
		let mut engine = Engine::new(); // Creating the Rhai engine
		let mut scope = Scope::new(); // Creatig the Rhai scope

		// Values available in the scope
		scope
			.push("attacks", Vec::<Dynamic>::new())
			.push_constant("player_pos", player.stats.get_pos().clone())
			.push_constant("enemy_pos", enemy.stats.get_pos().clone())
			.push_constant("target_pos", self.current_target.clone());

		// Values needed for the script, but not exposed to it 
		let enemy_pos = enemy.stats.get_pos();
		let enemy_id = enemy.get_id();

		// The Vec2 built-in methods don't work, so I have to make shitty copies
		fn move_towards(pos1: Vec2, pos2: Vec2, distance: f32) -> Vec2 {
			return pos1.move_towards(pos2, distance)
		}
		fn distance_between(pos1: Vec2, pos2: Vec2) -> f32 {
			return pos1.distance(pos2)
		}

		// Registerring functions for the script
		engine
			// Registerring the Vec2 and functions related to it
			.register_type_with_name::<Vec2>("position")
			.register_fn("move_towards", move_towards)
			.register_fn("distance_between", distance_between)

			// Functions for creating attacks
			.register_fn("new_physical", move |damage: i64, size| Attack::new_physical(
				enemy_pos, 
				damage as isize, 
				size, 
				Owner::Enemy(enemy_id)
			))
			.register_fn("new_burst", move |damage: i64, size| Attack::new_burst(
				enemy_pos, 
				damage as isize, 
				size, 
				Owner::Enemy(enemy_id)
			))
			.register_fn("new_projectile", move |damage: i64, target: Vec2| Attack::new_projectile(
				enemy_pos, 
				target,
				damage as isize, 
				Owner::Enemy(enemy_id)
			))
			.register_fn("new_hitscan", move |damage: i64, target: Vec2| Attack::new_hitscan(
				enemy_pos, 
				target,
				damage as isize, 
				Owner::Enemy(enemy_id)
			))

			// Hacky method to end the script
			.register_fn("end", move || Vec2::new(999999., 999999.));

		// Executing the script
		let new_pos = match engine.eval_with_scope::<Vec2>(&mut scope, &self.script) {
			Ok(new_pos) => new_pos,
			Err(e) => panic!("Bad script: {}", e)
		};

		// Getting attacks out of the scope
		let new_attacks = scope
			.get_value_mut::<Vec<Dynamic>>("attacks")
			.expect("Attacks not found");
		for i in new_attacks {
			attacks.push(i.clone_cast())
		}

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
