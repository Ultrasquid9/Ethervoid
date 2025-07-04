use std::{
	fs::File,
	path::{Path, PathBuf},
};

use bincode::{Decode, Encode, config, decode_from_slice, encode_to_vec};
use rustc_hash::FxHashSet;
use tracing::{error, info};

use crate::utils::random_seed;

#[derive(Encode, Decode)]
pub struct Save {
	pub seen_maps: FxHashSet<String>,
	pub seed: u64,
}

impl Save {
	pub fn read(dir: impl AsRef<Path>) -> Self {
		let file = match File::open(dir) {
			Ok(ok) => ok,
			Err(e) => {
				error!("Failed to open save file: {e}");
				return Self::default();
			}
		};

		let bytes = match zstd::decode_all(file) {
			Ok(ok) => ok,
			Err(e) => {
				error!("Failed to decompress save file: {e}");
				return Self::default();
			}
		};

		match decode_from_slice(&bytes, config::standard()) {
			Ok((ok, _)) => {
				info!("Save file loaded!");
				ok
			}
			Err(e) => {
				error!("Failed to decode save file: {e}");
				Self::default()
			}
		}
	}

	pub fn save(&self, dir: impl Into<PathBuf>) {
		let dir = dir.into();
		let bytes = match encode_to_vec(self, config::standard()) {
			Ok(ok) => ok,
			Err(e) => {
				error!("Failed to encode save: {e}");
				return;
			}
		};

		rayon::spawn(move || {
			let compressed = match zstd::bulk::compress(&bytes, 0) {
				Ok(ok) => ok,
				Err(e) => {
					error!("Failed to compress save: {e}");
					return;
				}
			};

			if let Err(e) = std::fs::write(dir, compressed) {
				error!("Failed to write save: {e}")
			} else {
				info!("Game saved successfully!")
			}
		});
	}
}

impl Default for Save {
	fn default() -> Self {
		Self {
			seen_maps: FxHashSet::default(),
			seed: random_seed(),
		}
	}
}
