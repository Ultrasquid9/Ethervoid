use ahash::HashMap;
use image::{DynamicImage, GenericImage};
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
	gameplay::doors::Door, 
	utils::{
		resources::textures::access_image, 
		vec2_to_tuple
	}
};

#[derive(Deserialize)]
struct MapBuilder {
	points: Vec<Vec2>,
	doors: Vec<Door>,
	enemies: Vec<(String, Vec2)>,
	npcs: Vec<(String, Vec2)>,
	tilemap: MapTexture
}

#[derive(Deserialize)]
struct MapTexture {
	keys: HashMap<char, String>,
	tiles: Vec<Vec<char>>
}

#[derive(Clone)]
pub struct Map {
	pub walls: Vec<Barrier>,
	pub doors: Vec<Door>,
	pub enemies: Vec<(EnemyType, Vec2)>,
	pub npcs: Vec<(NpcType, Vec2)>,
	pub texture: DynamicImage
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
				.collect(),

			texture: self.tilemap.to_texture()
		}
	}
}

impl MapTexture {
	fn to_texture(self) -> DynamicImage {
		let mut texture = DynamicImage::new_rgba8(
			(self.tiles[0].len() * 16) as u32, 
			(self.tiles.len() * 16) as u32
		);

		for i in 0..self.tiles.len() {
			let index_vert = i * 16;

			for (i, key) in self.tiles[i].iter().enumerate() {
				let index_hor = i * 16;

				let _ = texture.copy_from(
					&access_image(self.keys.get(key).unwrap()), 
					index_hor as u32, 
					index_vert as u32
				);
			}
		}

		texture
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
				println!("{}", mapbuilder.err().unwrap());
				None
			} else {
				Some((str, mapbuilder.unwrap().build(&enemytypes, &npctypes)))
			}
		})
		.collect();

	maps
}
