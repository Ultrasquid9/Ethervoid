use std::fs;

use serde_json::Value;

use crate::gameplay::enemy::{Attacks, Movement};

/// A struct containing the stats of an enemy type
pub struct EnemyBuilder {
	pub max_health: usize,
	pub movement: Movement,
	pub attacks: Vec<Attacks>
}

impl EnemyBuilder {
	pub fn new(dir: String) -> EnemyBuilder {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let builder = EnemyBuilder {
			max_health: input["Max Health"].as_u64().unwrap() as usize,
			movement: Movement::from_str(input["Movement"].as_str().unwrap()),
			attacks: Attacks::from_vec(input["Attacks"].as_array().unwrap())
		};

		return builder;
	}
}