use crate::{cores::textures::get_textures, gameplay::draw::process::downscale};
use ahash::HashMap;
use image::DynamicImage;
use imageproc::rgba_image;
use log::error;
use parking_lot::RwLock;
use std::sync::LazyLock;

/*
 *	Textures
 */

static TEXTURES: LazyLock<RwLock<HashMap<String, DynamicImage>>> =
	LazyLock::new(|| RwLock::new(HashMap::default()));

/// Populates the texture HashMap
pub(super) fn create_textures() {
	let textures = get_textures();

	for (key, texture) in textures {
		TEXTURES.write().insert(key, texture);
	}
}

/// Cleans the texture HashMap
pub(super) fn clean_textures() {
	TEXTURES.write().clear()
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	let thing = TEXTURES.read();
	let Some(texture) = thing.get(key) else {
		error!("Texture {} not found", key);
		return downscale(
			DynamicImage::ImageRgba8(rgba_image!(
				[0,0,0,255],[255,0,255,255];
				[255,0,255,255],[0,0,0,255]
			)),
			16,
		);
	};

	return texture.clone();
}
