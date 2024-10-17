use std::{collections::HashMap, fs};

use macroquad::math::Vec2;
use rapier2d::na::Point2;
use serde_json::Value;

use super::{enemybuilder::{get_enemybuilders, EnemyBuilder}, get_builders, get_name};

pub struct MapBuilder {
	pub points: Vec<Point2<f32>>,
	pub enemies: Vec<(EnemyBuilder, Vec2)>
}

impl MapBuilder {
	/// Creates a MapBuilder from the directory of the given string
	pub fn from(dir: String) -> MapBuilder {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let mut builder = MapBuilder {
			points: Vec::new(),
			enemies: Vec::new()
		};

		for i in input["Points"].as_array().unwrap() {
			let point = i.as_array().unwrap();
			builder.points.push(Point2::new(
				point[0].as_f64().unwrap() as f32, 
				point[1].as_f64().unwrap() as f32
			));
		}

		let enemybuilders = get_enemybuilders();

		for i in input["Enemies"].as_array().unwrap() {
			let enemy = i.as_array().unwrap();

			builder.enemies.push((
				enemybuilders.get(enemy[0].as_str().unwrap())
					.unwrap()
					.clone(), 
				Vec2::new(
					i[1].as_f64().unwrap() as f32, 
					i[2].as_f64().unwrap() as f32
				)
			));
		}

		return builder
	}
}

/// Provides a HashMap containing all MapBuilders
pub fn get_mapbuilders() -> HashMap<String, MapBuilder> {
	let mut builders: HashMap<String, MapBuilder> = HashMap::new();

	for i in get_builders(String::from("maps")) {
		builders.insert(
			get_name(&i),
			MapBuilder::from(i)
		);
	}

	return builders;
}
