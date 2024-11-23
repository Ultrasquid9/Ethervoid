use std::sync::{LazyLock, RwLock};

use ahash::HashMap;
use macroquad::rand;

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
static AUDIO: LazyLock<RwLock<HashMap<String, StaticSoundData>>> = LazyLock::new(|| RwLock::new(HashMap::default()));

/// Populates the Audio HashMap
pub(super) fn create_audio() {
	let audio = get_audio();

	for i in audio {
		AUDIO.write().unwrap().insert(i.0, i.1);
	}
}

/// Cleans the Audio HashMap
pub(super) fn clean_audio() {
	AUDIO.write().unwrap().clear();
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) {
	MANAGER.write().unwrap().play(AUDIO.read().unwrap().get(key).unwrap().clone()).unwrap();
}

/// Plays a random sound from the provided list of keys 
pub fn play_random_sound(keys: &[&str]) {
	play_sound(keys[rand::gen_range(0, keys.len() - 1)]);
}
