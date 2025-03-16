use raywoke::Barrier;
use serde::Deserialize;

use super::{
	Readable,
	enemytype::{EnemyType, get_enemytypes},
	gen_name, get_files,
	npctype::{NpcType, get_npctypes},
};

use crate::{gameplay::doors::Door, prelude::*, utils::resources::textures::access_image};

use image::{DynamicImage, GenericImage};

#[derive(Deserialize)]
struct MapBuilder {
	points: Vec<DVec2>,
	doors: Vec<Door>,
	enemies: Vec<(String, DVec2)>,
	npcs: Vec<(String, DVec2)>,
	tilemap: MapTexture,
}

#[derive(Deserialize)]
struct MapTexture {
	keys: HashMap<char, String>,
	tiles: Vec<Vec<char>>,
}

#[derive(Clone)]
pub struct Map {
	pub walls: Box<[Barrier]>,
	pub doors: Box<[Door]>,
	pub enemies: Box<[(EnemyType, DVec2)]>,
	pub npcs: Box<[(NpcType, DVec2)]>,
	pub texture: DynamicImage,
}

impl Readable for MapBuilder {}

impl MapBuilder {
	pub fn build(
		self,
		enemytypes: &HashMap<String, EnemyType>,
		npctypes: &HashMap<String, NpcType>,
	) -> Map {
		Map {
			walls: {
				let mut walls = Vec::new();

				for point in 0..self.points.len() {
					match self.points.get(point + 1) {
						Some(_) => walls.push(Barrier::new(
							self.points.get(point).unwrap().as_vec2(),
							self.points.get(point + 1).unwrap().as_vec2(),
						)),
						None => walls.push(Barrier::new(
							self.points.get(point).unwrap().as_vec2(),
							self.points.first().unwrap().as_vec2(),
						)),
					}
				}

				walls.into_boxed_slice()
			},
			doors: self.doors.into_boxed_slice(),

			enemies: self
				.enemies
				.par_iter()
				.map(|(name, pos)| (enemytypes.get(name.as_str()).unwrap().clone(), *pos))
				.collect(),

			npcs: self
				.npcs
				.par_iter()
				.map(|(name, pos)| (npctypes.get(name.as_str()).unwrap().clone(), *pos))
				.collect(),

			texture: self.tilemap.to_texture(),
		}
	}
}

impl MapTexture {
	fn to_texture(&self) -> DynamicImage {
		let mut texture = DynamicImage::new_rgba8(
			(self.tiles[0].len() * 16) as u32,
			(self.tiles.len() * 16) as u32,
		);

		for i in 0..self.tiles.len() {
			let index_vert = i * 16;

			for (i, key) in self.tiles[i].iter().enumerate() {
				let index_hor = i * 16;

				let _ = texture.copy_from(
					&access_image(self.keys.get(key).unwrap()),
					index_hor as u32,
					index_vert as u32,
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
				warn!("Map {} failed to load: {}", str, mapbuilder.err().unwrap());
				None
			} else {
				info!("Map {} loaded!", str);
				Some((str, mapbuilder.unwrap().build(&enemytypes, &npctypes)))
			}
		})
		.collect();

	maps
}
