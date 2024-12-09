use std::fs;
use ahash::HashMap;
use serde::Deserialize;
use rayon::prelude::*;

use super::{
	script::{
		get_scripts, 
		ScriptBuilder
	}, 
	gen_name, 
	get_files
};

#[derive(Clone, Deserialize)]
struct EnemyTypeBuilder {
	max_health: f32,
	size: f32,
	sprite: String,
	movement: String,
	attacks: Vec<String>
}

impl EnemyTypeBuilder {
	pub fn read(dir: &str) -> Self {
		ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap()
	}

	pub fn build(self) -> EnemyType {
		let scripts = get_scripts();

		EnemyType {
			max_health: self.max_health,
			size: self.size,
			sprite: self.sprite,
			movement: scripts.get(&self.movement).unwrap().clone(), 
			attacks: self.attacks
				.par_iter()
				.map(|attack| scripts.get(attack.as_str()).unwrap().clone())
				.collect()
		}
	}
}

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyType {
	pub max_health: f32,
	pub size: f32,
	pub sprite: String,
	pub movement: ScriptBuilder,
	pub attacks: Vec<ScriptBuilder>
}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let enemytypes: HashMap<String, EnemyType> = get_files("enemies".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), EnemyTypeBuilder::read(dir).build()))
		.collect();

	enemytypes
}
