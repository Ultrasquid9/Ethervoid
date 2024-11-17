// This file contains globally available resources
// Everyone always says "don't do this" so fuck you I did

use std::sync::RwLock;

use ahash::HashMap;
use image::DynamicImage;
use macroquad::texture::Texture2D;
use once_cell::sync::Lazy;

use crate::gameplay::{cores::textures::get_textures, draw::textures::to_texture};

// Textures

static TEXTURES: Lazy<RwLock<HashMap<String, DynamicImage>>> = Lazy::new(|| RwLock::new(HashMap::default()));

/// Populates the texture HashMap
/// NOTE: Please ensure you call `clean_attack_textures()` when quitting the game.
pub fn create_textures() {
	let textures = get_textures();

	for i in textures {
		TEXTURES.write().unwrap().insert(i.0, i.1);
	}
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	TEXTURES.read().unwrap().get(key).unwrap().clone()
}

/// Gets the texture at the provided key
pub fn access_texture(key: &str) -> Texture2D {
	to_texture(access_image(key))
}

/// Clears the texture HashMap
pub fn clean_textures() {
	TEXTURES.write().unwrap().clear()
}
