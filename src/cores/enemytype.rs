use log::warn;
use serde::Deserialize;

use super::{Readable, gen_name, get_files};

use crate::{gameplay::ecs::sprite::Frames, prelude::*};

/// A struct containing the stats of an enemy type
#[derive(Clone, Deserialize)]
pub struct EnemyType {
	pub max_health: f64,
	pub size: f64,
	pub sprite: String,
	pub goals: Box<[String]>,
	pub anims: HashMap<String, Frames>,
}

impl Readable for EnemyType {}

/// Provides a HashMap containing all EnemyTypes
pub fn get_enemytypes() -> HashMap<String, EnemyType> {
	let enemytypes: HashMap<String, EnemyType> = get_files("enemies".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), EnemyType::read(dir)))
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
				Some((str, enemytypebuilder.unwrap()))
			}
		})
		.collect();

	enemytypes
}
