use std::fs;

use rapier2d::na::Point2;
use serde_json::Value;

use super::{enemybuilder::EnemyBuilder, get_builders};

pub struct MapBuilder {
	points: Vec<Point2<f32>>,
	enemies: Vec<EnemyBuilder>
}

impl MapBuilder {
	pub fn from(dir: String) -> MapBuilder {
		let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();

		let mut builder = MapBuilder {
			points: Vec::new(),
			enemies: Vec::new()
		};

		for i in input["Points"].as_array().unwrap() {
			let point = i.as_array().unwrap();
			builder.points.push(Point2::new(point[0].as_f64().unwrap() as f32, point[1].as_f64().unwrap() as f32));
		}

		return builder
	}
}

pub fn get_mapbuilders() -> Vec<MapBuilder> {
	let mut builders: Vec<MapBuilder> = Vec::new();

	for i in get_builders(String::from("maps")) {
		builders.push(MapBuilder::from(i));
	}

	return builders;
}
