use ahash::HashMap;
use macroquad::math::Vec2;
use raylite::Barrier;
use serde::Deserialize;
use rayon::prelude::*;

use super::{
	enemytype::{
		get_enemytypes, 
		EnemyType
	}, gen_name, get_files, npctype::{
		get_npctypes, 
		NpcType
	}, Readable
};

use crate::{
	utils::vec2_to_tuple,
	gameplay::doors::Door
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
	pub walls: Vec<Barrier>,
	pub doors: Vec<Door>,
	pub enemies: Vec<(EnemyType, Vec2)>,
	pub npcs: Vec<(NpcType, Vec2)>
}

impl Readable for MapBuilder {}

impl MapBuilder {
	pub fn build(self, enemytypes: &HashMap<String, EnemyType>, npctypes: &HashMap<String, NpcType>) -> Map {
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
			doors: self.doors,

			enemies: self.enemies
				.par_iter()
				.map(|enemy| (
					enemytypes.get(enemy.0.as_str()).unwrap().clone(),
					enemy.1
				))
				.collect(),
			
			npcs: self.npcs
				.par_iter()
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
	let enemytypes = get_enemytypes();
	let npctypes = get_npctypes();

	let maps: HashMap<String, Map> = get_files("maps".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), MapBuilder::read(dir)))
		.filter_map(|(str, mapbuilder)| {
			if mapbuilder.is_err() {
				None
			} else {
				Some((str, mapbuilder.unwrap().build(&enemytypes, &npctypes)))
			}
		})
		.collect();

	maps
}
