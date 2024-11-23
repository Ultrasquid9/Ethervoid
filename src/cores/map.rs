use std::fs;

use ahash::HashMap;
use macroquad::math::Vec2;
use raylite::Barrier;
use serde::Deserialize;

use crate::utils::vec2_to_tuple;

use super::{
	enemytype::{
		get_enemytypes, 
		EnemyType
	}, 
	npctype::{
		get_npctypes, 
		NpcType
	},
	gen_name, 
	get_files
};

#[derive(Deserialize)]
struct MapBuilder {
	pub points: Vec<Vec2>,
	//pub doors: Vec<Door>,
	pub enemies: Vec<(String, Vec2)>,
	pub npcs: Vec<(String, Vec2)>
}

#[derive(Clone)]
pub struct Map {
	pub walls: Vec<Barrier>,
	//pub doors: Vec<Door>,
	pub enemies: Vec<(EnemyType, Vec2)>,
	pub npcs: Vec<(NpcType, Vec2)>
}

impl MapBuilder {
	pub fn read(dir: String) -> Self {
		ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap()
	}

	pub fn build(self) -> Map {
		let enemytypes = get_enemytypes();
		let npctypes = get_npctypes();

		Map {
			walls: {
				let mut walls = Vec::new();

				for point in 0..self.points.len() {
					match self.points.get(point + 1) {
						Some(_) => walls.push(Barrier {
							positions: (vec2_to_tuple(
								self.points.get(point).unwrap()), 
								vec2_to_tuple(self.points.get(point + 1).unwrap())
							)
						}),
						None => walls.push(Barrier {
							positions: (
								vec2_to_tuple(self.points.get(point).unwrap()), 
								(vec2_to_tuple(self.points.first().unwrap()))
							)
						})
					}
				}

				walls
			},
			//doors: self.doors,

			enemies: self.enemies
				.iter()
				.map(|enemy| (
					enemytypes.get(enemy.0.as_str()).unwrap().clone(),
					enemy.1
				))
				.collect(),
			
			npcs: self.npcs
				.iter()
				.map(|npc| (
					npctypes.get(npc.0.as_str()).unwrap().clone(),
					npc.1
				))
				.collect()
		}
	}
}

/// Provides a HashMap containing all Maps
pub fn get_maps() -> HashMap<String, Map> {
	let mut maps: HashMap<String, Map> = HashMap::default();

	for i in get_files(String::from("maps")) {
		maps.insert(
			gen_name(&i),
			MapBuilder::read(i).build()
		);
	}

	maps
}
