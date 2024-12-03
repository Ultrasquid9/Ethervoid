use ahash::HashMap;
use image::DynamicImage;

use crate::cores::textures::get_textures; 

use std::sync::{
	LazyLock, 
	RwLock
};

/*
 *	Textures
 */

static TEXTURES: LazyLock<RwLock<HashMap<String, DynamicImage>>> = LazyLock::new(|| RwLock::new(HashMap::default()));

/// Populates the texture HashMap
pub(super) fn create_textures() {
	let textures = get_textures();

	for (key, texture) in textures {
		TEXTURES.write().unwrap().insert(key, texture);
	}
}

/// Cleans the texture HashMap
pub(super) fn clean_textures() {
	TEXTURES.write().unwrap().clear()
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	return TEXTURES.read().unwrap().get(key).unwrap().clone()
}
