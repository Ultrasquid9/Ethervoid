use log::error;
use macroquad::rand;
use parking_lot::RwLock;
use std::sync::LazyLock;

use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};

use crate::cores::audio::get_audio;

use super::{Resource, resource};

/*
 *	Audio
 */

static MANAGER: LazyLock<RwLock<AudioManager>> = LazyLock::new(|| {
	RwLock::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap())
});
static SOUNDS: Resource<StaticSoundData> = resource();

/// Populates the Sounds HashMap
pub(super) fn create_sounds() {
	let mut access = SOUNDS.write();
	access.clear();
	*access = get_audio();
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) {
	let thing = SOUNDS.read();

	let Some(sound) = thing.get(key) else {
		error!("Sound {key} not found");
		return;
	};

	MANAGER.write().play(sound.clone()).unwrap();
}

/// Plays a random sound from the provided list of keys
pub fn play_random_sound(keys: &[&str]) {
	play_sound(keys[rand::gen_range(0, keys.len() - 1)]);
}
