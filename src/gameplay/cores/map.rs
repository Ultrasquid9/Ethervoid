use std::{collections::HashMap, fs};

use macroquad::math::Vec2;
use serde_json::Value;

use super::{enemytype::{get_enemytypes, EnemyType}, get_files, get_name};

pub struct Map {
	pub points: Vec<Vec2>,
	pub enemies: Vec<(EnemyType, Vec2)>
}

impl Map {
	/// Creates a Map from the directory of the given string
	pub fn from(dir: String) -> Map {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let mut map = Map {
			points: Vec::new(),
			enemies: Vec::new()
		};

		for i in input["Points"].as_array().unwrap() {
			let point = i.as_array().unwrap();
			map.points.push(Vec2::new(
				point[0].as_f64().unwrap() as f32, 
				point[1].as_f64().unwrap() as f32
			));
		}

		let enemytypes = get_enemytypes();

		for i in input["Enemies"].as_array().unwrap() {
			let enemy = i.as_array().unwrap();

			map.enemies.push((
				enemytypes.get(enemy[0].as_str().unwrap())
					.unwrap()
					.clone(), 
				Vec2::new(
					i[1].as_f64().unwrap() as f32, 
					i[2].as_f64().unwrap() as f32
				)
			));
		}

		return map
	}
}

/// Provides a HashMap containing all Maps
pub fn get_maps() -> HashMap<String, Map> {
	let mut maps: HashMap<String, Map> = HashMap::new();

	for i in get_files(String::from("maps")) {
		maps.insert(
			get_name(&i),
			Map::from(i)
		);
	}

	return maps;
}
