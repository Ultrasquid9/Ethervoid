use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::gameplay::enemy::Movement;

use super::{attack::{get_attacks, Attack}, get_files, get_name};

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyType {
	pub max_health: usize,
	pub size: f32,
	pub movement: Movement,
	pub attacks: Vec<Attack>
}

impl EnemyType {
	/// Creates an EnemyType from the directory of the given string
	pub fn from(dir: String) -> EnemyType {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let mut enemytype = EnemyType {
			max_health: input["Max Health"].as_u64().unwrap() as usize,
			size: input["Size"].as_f64().unwrap() as f32,
			movement: Movement::from_str(input["Movement"].as_str().unwrap()),
			attacks: Vec::new()
		};

		let attacks = get_attacks();

		for i in input["Attacks"].as_array().unwrap() {
			let attack = i.as_array().unwrap();

			enemytype.attacks.push(
				attacks.get(attack[0].as_str().unwrap())
					.unwrap()
					.clone()
			);
		}

		return enemytype;
	}
}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let mut enemytypes: HashMap<String, EnemyType> = HashMap::new();

	for i in get_files(String::from("enemies")) {
		enemytypes.insert(
			get_name(&i),
			EnemyType::from(i)
		);
	}

	return enemytypes;
}
