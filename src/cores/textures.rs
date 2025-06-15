use crate::prelude::*;
use std::sync::mpsc;

use imageproc::image::{ColorType, DynamicImage, ImageReader};

use super::{gen_name, get_files};

/// Provides a `HashMap` containing all Textures
pub fn get_textures() -> FxHashMap<String, DynamicImage> {
	let mut textures: FxHashMap<String, DynamicImage> = FxHashMap::default();
	let (transciever, receiver) = mpsc::channel();

	for dir in get_files("sprites") {
		let name: String = gen_name(&dir);

		macro_rules! maybe {
			($input:expr) => {
				match $input {
					Err(e) => {
						warn!("Texture {name} failed to load: {e}");
						continue;
					}
					Ok(ok) => ok,
				}
			};
		}

		let img = maybe!(maybe!(ImageReader::open(dir)).decode());
		info!("Texture {name} loaded!");

		_ = transciever.send((
			name,
			if img.color() == ColorType::Rgba8 {
				img
			} else {
				DynamicImage::ImageRgba8(img.to_rgba8())
			},
		));
	}

	drop(transciever);

	for (name, image) in receiver {
		textures.insert(name, image);
	}

	textures
}
