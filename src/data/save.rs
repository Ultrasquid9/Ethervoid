use std::{
	fs::File,
	path::{Path, PathBuf},
};

use bincode::{Decode, Encode, config, decode_from_slice, encode_to_vec};
use rustc_hash::FxHashSet;
use tracing::{error, info};

#[derive(Encode, Decode, Default)]
pub struct Save {
	pub seen_maps: FxHashSet<String>,
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
			Ok((ok, _)) => ok,
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

		std::thread::spawn(move || {
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
