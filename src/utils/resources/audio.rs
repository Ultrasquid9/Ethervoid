use macroquad::rand;
use parking_lot::RwLock;
use std::sync::LazyLock;
use tracing::error;

use kira::{AudioManager, AudioManagerSettings, sound::static_sound::StaticSoundData};

use crate::cores::audio::get_audio;

use super::{Resource, resource, set_resource};

/*
 *	Audio
 */

static MANAGER: LazyLock<RwLock<AudioManager>> = LazyLock::new(init_manager);
static SOUNDS: Resource<StaticSoundData> = resource();

/// Populates the Sounds `HashMap`
pub(super) fn create_sounds() {
	set_resource(&SOUNDS, get_audio());
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) {
	let thing = SOUNDS.read();

	let Some(sound) = thing.get(key) else {
		error!("Sound {key} not found");
		return;
	};

	if let Err(e) = MANAGER.write().play(sound.clone()) {
		error!("Error playing sound: {e}")
	}
}

/// Plays a random sound from the provided list of keys
pub fn play_random_sound(keys: &[&str]) {
	play_sound(keys[rand::gen_range(0, keys.len() - 1)]);
}

fn init_manager() -> RwLock<AudioManager> {
	RwLock::new(match AudioManager::new(AudioManagerSettings::default()) {
		Ok(manager) => manager,
		Err(e) => {
			error!("Audio Manager could not be created: {e}");
			panic!()
		}
	})
}
