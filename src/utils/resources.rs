use std::sync::RwLock;
use ahash::HashMap;
use image::DynamicImage;
use macroquad::texture::Texture2D;
use once_cell::sync::Lazy;

use kira::{
	manager::{
		AudioManager, 
		AudioManagerSettings, 
		DefaultBackend
	}, 
	sound::static_sound::StaticSoundData
};

use crate::gameplay::{
	cores::{
		audio::get_audio, 
		textures::get_textures
	},
	draw::textures::to_texture
};

// This file contains globally available resources
// Everyone always says "don't do this" so fuck you I did

/// Populates global resources
/// NOTE: Please ensure you call `clean_attack_textures()` when quitting the game.
pub fn create_resources() {
	create_audio();
	create_textures();
}

/// Cleans the global resources
pub fn clean_resources() {
	clean_audio();
	clean_textures();
}

/*
 *	Textures
 */ 

static TEXTURES: Lazy<RwLock<HashMap<String, DynamicImage>>> = Lazy::new(|| RwLock::new(HashMap::default()));

/// Populates the texture HashMap
fn create_textures() {
	let textures = get_textures();

	for i in textures {
		TEXTURES.write().unwrap().insert(i.0, i.1);
	}
}

/// Cleans the texture HashMap
fn clean_textures() {
	TEXTURES.write().unwrap().clear()
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	TEXTURES.read().unwrap().get(key).unwrap().clone()
}

/// Gets the texture at the provided key
pub fn access_texture(key: &str) -> Texture2D {
	to_texture(access_image(key))
}

/*
 *	Audio
 */ 

static MANAGER: Lazy<RwLock<AudioManager>> = Lazy::new(|| RwLock::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap()));
static AUDIO: Lazy<RwLock<HashMap<String, StaticSoundData>>> = Lazy::new(|| RwLock::new(HashMap::default()));

/// Populates the Audio HashMap
fn create_audio() {
	let audio = get_audio();

	for i in audio {
		AUDIO.write().unwrap().insert(i.0, i.1);
	}
}

/// Cleans the Audio HashMap
fn clean_audio() {
	AUDIO.write().unwrap().clear();
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) {
	MANAGER.write().unwrap().play(AUDIO.read().unwrap().get(key).unwrap().clone()).unwrap();
}
