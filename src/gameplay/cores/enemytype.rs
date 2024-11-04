use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::gameplay::enemy::Movement;

use super::{attackscript::{get_attacks, AttackScriptBuilder}, gen_name, get_files};

#[derive(Clone, Deserialize)]
struct EnemyTypeBuilder {
	max_health: usize,
	size: f32,
	movement: String,
	attacks: Vec<String>
}

impl EnemyTypeBuilder {
	pub fn read(dir: String) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}

	pub fn build(self) -> EnemyType {
		let attacks = get_attacks();

		EnemyType {
			max_health: self.max_health,
			size: self.size,
			movement: Movement::from_str(&self.movement), 
			attacks: self.attacks
				.iter()
				.map(|attack| attacks.get(attack.as_str()).unwrap().clone())
				.collect()
		}
	}
}

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyType {
	pub max_health: usize,
	pub size: f32,
	pub movement: Movement,
	pub attacks: Vec<AttackScriptBuilder>
}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let mut enemytypes: HashMap<String, EnemyType> = HashMap::new();

	for i in get_files(String::from("enemies")) {
		enemytypes.insert(
			gen_name(&i),
			EnemyTypeBuilder::read(i).build()
		);
	}

	return enemytypes;
}
