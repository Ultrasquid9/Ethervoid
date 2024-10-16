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

/// Creates a vec of EnemyBuilders containing all EnemyBuilders in all cores
pub fn get_enemy_builders() -> Vec<EnemyBuilder> {
	// This function took way too long to write
	
	let mut builders: Vec<EnemyBuilder> = Vec::new();

	let mut enemies_paths: Vec<String> = Vec::new();

	for i in fs::read_dir("./cores").unwrap() {
		let dir = i.unwrap().file_name().to_string_lossy().into_owned();
		enemies_paths.push(dir);
	}

	for i in enemies_paths {
		for j in fs::read_dir(format!("./cores/{}/enemies", i).as_str()).unwrap() {
			builders.push(EnemyBuilder::new(format!("./cores/{}/enemies/{}", i, j.unwrap().file_name().to_string_lossy().into_owned())));
		}
	}

	return builders;
}
