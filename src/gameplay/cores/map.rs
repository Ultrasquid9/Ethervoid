use std::{collections::HashMap, fs};

use macroquad::math::Vec2;
use serde::Deserialize;

use super::{enemytype::{get_enemytypes, EnemyType}, gen_name, get_files};

#[derive(Deserialize)]
struct MapBuilder {
	pub points: Vec<Vec2>,
	pub enemies: Vec<(String, Vec2)>
}

pub struct Map {
	pub points: Vec<Vec2>,
	pub enemies: Vec<(EnemyType, Vec2)>
}

impl MapBuilder {
	pub fn read(dir: String) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}

	pub fn build(self) -> Map {
		let enemytypes = get_enemytypes();

		Map {
			points: self.points,
			enemies: self.enemies
				.iter()
				.map(|enemy| (
					enemytypes.get(enemy.0.as_str()).unwrap().clone(),
					enemy.1
				))
				.collect()
		}
	}
}

/// Provides a HashMap containing all Maps
pub fn get_maps() -> HashMap<String, Map> {
	let mut maps: HashMap<String, Map> = HashMap::new();

	for i in get_files(String::from("maps")) {
		maps.insert(
			gen_name(&i),
			MapBuilder::read(i).build()
		);
	}

	return maps;
}
