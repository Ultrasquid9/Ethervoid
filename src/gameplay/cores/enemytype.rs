use std::fs;
use ahash::HashMap;
use serde::Deserialize;

use super::{
	behavior::{
		get_attacks, 
		BehaviorBuilder
	}, 
	gen_name, 
	get_files
};

#[derive(Clone, Deserialize)]
struct EnemyTypeBuilder {
	max_health: usize,
	size: f32,
	sprite: String,
	movement: String,
	attacks: Vec<String>
}

impl EnemyTypeBuilder {
	pub fn read(dir: String) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}

	pub fn build(self) -> EnemyType {
		let behaviors = get_attacks();

		EnemyType {
			max_health: self.max_health,
			size: self.size,
			sprite: self.sprite,
			movement: behaviors.get(&self.movement).unwrap().clone(), 
			attacks: self.attacks
				.iter()
				.map(|attack| behaviors.get(attack.as_str()).unwrap().clone())
				.collect()
		}
	}
}

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyType {
	pub max_health: usize,
	pub size: f32,
	pub sprite: String,
	pub movement: BehaviorBuilder,
	pub attacks: Vec<BehaviorBuilder>
}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let mut enemytypes: HashMap<String, EnemyType> = HashMap::default();

	for i in get_files(String::from("enemies")) {
		enemytypes.insert(
			gen_name(&i),
			EnemyTypeBuilder::read(i).build()
		);
	}

	return enemytypes;
}
