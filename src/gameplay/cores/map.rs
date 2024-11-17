use std::fs;

use ahash::HashMap;
use macroquad::math::Vec2;
use serde::Deserialize;

use crate::gameplay::doors::Door;

use super::{
	enemytype::{
		get_enemytypes, 
		EnemyType
	}, 
	npctype::{
		get_npctypes, 
		NPCType
	},
	gen_name, 
	get_files
};

#[derive(Deserialize)]
struct MapBuilder {
	pub points: Vec<Vec2>,
	pub doors: Vec<Door>,
	pub enemies: Vec<(String, Vec2)>,
	pub npcs: Vec<(String, Vec2)>
}

#[derive(Clone)]
pub struct Map {
	pub points: Vec<Vec2>,
	pub doors: Vec<Door>,
	pub enemies: Vec<(EnemyType, Vec2)>,
	pub npcs: Vec<(NPCType, Vec2)>
}

impl MapBuilder {
	pub fn read(dir: String) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}

	pub fn build(self) -> Map {
		let enemytypes = get_enemytypes();
		let npctypes = get_npctypes();

		Map {
			points: self.points,
			doors: self.doors,

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

	return maps;
}
