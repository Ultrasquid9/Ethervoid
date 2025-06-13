use raywoke::Barrier;
use serde::Deserialize;

use super::{
	enemytype::{EnemyType, get_enemytypes},
	gen_name, get_files,
	npctype::{NpcType, get_npctypes},
	read_from_path,
};

use crate::{
	gameplay::{doors::Door, draw::process::to_texture},
	prelude::*,
	utils::{ImmutVec, resources::textures::access_image, tup_vec::Tup64},
};

use image::{DynamicImage, GenericImage};

#[derive(Deserialize)]
struct MapBuilder {
	walls: Vec<Vec<DVec2>>,
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
	pub walls: ImmutVec<ImmutVec<Barrier>>,
	pub doors: ImmutVec<Door>,
	pub enemies: ImmutVec<(EnemyType, DVec2)>,
	pub npcs: ImmutVec<(NpcType, DVec2)>,
	pub texture: Texture2D,
}

impl MapBuilder {
	pub fn build(
		self,
		enemytypes: &HashMap<String, EnemyType>,
		npctypes: &HashMap<String, NpcType>,
	) -> Map {
		// Handles the iterator chain for enemies/npcs
		fn iter_thing<T: Clone>(
			input: &[(String, DVec2)],
			hashmap: &HashMap<String, T>,
			type_name: &str,
		) -> ImmutVec<(T, DVec2)> {
			input
				.iter()
				.map(|(name, pos)| (name, hashmap.get(name.as_str()), pos))
				.filter_map(|(name, opt, pos)| {
					if let Some(t) = opt {
						Some((t.clone(), *pos))
					} else {
						error!("{type_name} {name} not found! Skipping...");
						None
					}
				})
				.collect()
		}

		Map {
			walls: {
				let mut walls = vec![];

				for wall in self.walls {
					let bar = |start: usize, end: usize| {
						Barrier::new(wall[start].tup64(), wall[end].tup64())
					};

					let mut vec = vec![];

					for point in 0..wall.len() {
						match wall.get(point + 1) {
							Some(_) => vec.push(bar(point, point + 1)),
							None => vec.push(bar(point, 0)),
						}
					}

					walls.push(vec.into_boxed_slice());
				}

				walls.into_boxed_slice()
			},
			doors: self.doors.into_boxed_slice(),

			enemies: iter_thing(&self.enemies, enemytypes, "EnemyType"),
			npcs: iter_thing(&self.npcs, npctypes, "NpcType"),

			texture: self.tilemap.to_texture(),
		}
	}
}

impl MapTexture {
	fn to_texture(&self) -> Texture2D {
		let mut texture = DynamicImage::new_rgba8(
			(self.tiles[0].len() * 16) as u32,
			(self.tiles.len() * 16) as u32,
		);

		for i in 0..self.tiles.len() {
			let index_vert = i * 16;

			for (i, key) in self.tiles[i].iter().enumerate() {
				let index_hor = i * 16;

				_ = texture.copy_from(
					access_image(self.keys.get(key).unwrap_or(&String::new())),
					index_hor as u32,
					index_vert as u32,
				);
			}
		}

		to_texture(&texture)
	}
}

/// Provides a `HashMap` containing all Maps
pub fn get_maps() -> HashMap<String, Map> {
	let enemytypes = get_enemytypes();
	let npctypes = get_npctypes();

	get_files("maps")
		.iter()
		.map(|dir| (gen_name(dir), read_from_path::<MapBuilder>(dir)))
		.filter_map(|(str, result)| match result {
			Err(e) => {
				warn!("Map {str} failed to load: {e}");
				None
			}
			Ok(map) => {
				info!("Map {str} loaded!");
				Some((str, map.build(&enemytypes, &npctypes)))
			}
		})
		.collect()
}
