use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::gameplay::enemy::{Attacks, Movement};

use super::{get_builders, get_name};

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyBuilder {
	pub max_health: usize,
	pub size: f32,
	pub movement: Movement,
	pub attacks: Vec<Attacks>
}

impl EnemyBuilder {
	/// Creates an EnemyBuilder from the directory of the given string
	pub fn from(dir: String) -> EnemyBuilder {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let builder = EnemyBuilder {
			max_health: input["Max Health"].as_u64().unwrap() as usize,
			size: input["Size"].as_f64().unwrap() as f32,
			movement: Movement::from_str(input["Movement"].as_str().unwrap()),
			attacks: Attacks::from_vec(input["Attacks"].as_array().unwrap())
		};

		return builder;
	}
}

/// Provides a HashMap containing all EnemyBuilders
pub fn get_enemybuilders() -> HashMap<String, EnemyBuilder> {
	let mut builders: HashMap<String, EnemyBuilder> = HashMap::new();

	for i in get_builders(String::from("enemies")) {
		builders.insert(
			get_name(&i),
			EnemyBuilder::from(i)
		);
	}

	return builders;
}
