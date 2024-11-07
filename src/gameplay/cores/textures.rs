use std::collections::HashMap;

use image::{DynamicImage, ImageReader};

use super::{gen_name, get_files};

/// Provides a HashMap containing all Textures
/// TODO: Support sub-directories
pub fn get_textures() -> HashMap<String, DynamicImage> {
	let mut textures: HashMap<String, DynamicImage> = HashMap::new();

	for i in get_files(String::from("sprites")) {
		textures.insert(
			gen_name(&i),
			ImageReader::open(&i)
				.unwrap()
				.decode()
				.unwrap()
		);
	}

	return textures;
}