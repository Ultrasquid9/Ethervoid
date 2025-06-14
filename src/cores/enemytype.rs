use serde::Deserialize;
use tracing::warn;

use super::{gen_name, get_files, read_from_path};

use crate::{gameplay::ecs::sprite::Frames, prelude::*, utils::ImmutVec};

/// A struct containing the stats of an enemy type
#[derive(Clone, Deserialize)]
pub struct EnemyType {
	pub max_health: f64,
	pub size: f64,
	pub sprite: String,
	pub goals: ImmutVec<String>,
	pub anims: FxHashMap<String, Frames>,
}

/// Provides a `HashMap` containing all `EnemyTypes`
pub fn get_enemytypes() -> FxHashMap<String, EnemyType> {
	let enemytypes: FxHashMap<String, EnemyType> = get_files("enemies")
		.iter()
		.map(|dir| (gen_name(dir), read_from_path(dir)))
		.filter_map(|(str, result)| match result {
			Err(e) => {
				warn!("EnemyType {str} failed to load: {e}");
				None
			}
			Ok(enemytype) => {
				info!("EnemyType {str} loaded!");
				Some((str, enemytype))
			}
		})
		.collect();

	enemytypes
}
