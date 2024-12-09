use std::fs;
use ahash::HashMap;
use macroquad::math::Vec2;
use raylite::Barrier;
use serde::Deserialize;
use rayon::prelude::*;

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

impl MapBuilder {
	pub fn read(dir: &str) -> Self {
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
	let maps: HashMap<String, Map> = get_files("maps".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), MapBuilder::read(dir).build()))
		.collect();

	maps
}
