use serde::Deserialize;

use super::{
	Readable, gen_name, get_files,
	goal::{GoalBuilder, get_goals},
};

use crate::{gameplay::ecs::sprite::Frames, prelude::*};

#[derive(Clone, Deserialize)]
struct EnemyTypeBuilder {
	max_health: f64,
	size: f64,
	sprite: String,
	goals: Vec<String>,
	anims: HashMap<String, Frames>,
}

impl Readable for EnemyTypeBuilder {}

impl EnemyTypeBuilder {
	pub fn build(self, scripts: &HashMap<String, GoalBuilder>) -> EnemyType {
		EnemyType {
			max_health: self.max_health,
			size: self.size,
			sprite: self.sprite,
			goals: self
				.goals
				.par_iter()
				.map(|attack| scripts.get(attack.as_str()).unwrap().clone())
				.collect(),
			anims: self.anims,
		}
	}
}

/// A struct containing the stats of an enemy type
#[derive(Clone)]
pub struct EnemyType {
	pub max_health: f64,
	pub size: f64,
	pub sprite: String,
	pub goals: Box<[GoalBuilder]>,
	pub anims: HashMap<String, Frames>,
}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let scripts = get_goals();

	let enemytypes: HashMap<String, EnemyType> = get_files("enemies".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), EnemyTypeBuilder::read(dir)))
		.filter_map(|(str, enemytypebuilder)| {
			if enemytypebuilder.is_err() {
				warn!(
					"EnemyType {} failed to load: {}",
					str,
					enemytypebuilder.err().unwrap()
				);
				None
			} else {
				info!("EnemyType {} loaded!", str);
				Some((str, enemytypebuilder.unwrap().build(&scripts)))
			}
		})
		.collect();

	enemytypes
}
