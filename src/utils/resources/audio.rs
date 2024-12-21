use ahash::HashMap;
use macroquad::rand;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	manager::{
		AudioManager, 
		AudioManagerSettings, 
		DefaultBackend
	}, 
	sound::static_sound::StaticSoundData
};

use crate::cores::audio::get_audio;

/*
 *	Audio
 */ 

static MANAGER: LazyLock<RwLock<AudioManager>> = LazyLock::new(|| RwLock::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap()));
static SOUNDS: LazyLock<RwLock<HashMap<String, StaticSoundData>>> = LazyLock::new(|| RwLock::new(HashMap::default()));

/// Populates the Sounds HashMap
pub(super) fn create_sounds() {
	let sounds = get_audio();

	for (key, sound) in sounds {
		SOUNDS.write().insert(key, sound);
	}
}

/// Cleans the Sounds HashMap
pub(super) fn clean_sounds() {
	SOUNDS.write().clear();
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) {
	MANAGER.write().play(SOUNDS.read().get(key).unwrap().clone()).unwrap();
}

/// Plays a random sound from the provided list of keys 
pub fn play_random_sound(keys: &[&str]) {
	play_sound(keys[rand::gen_range(0, keys.len() - 1)]);
}
