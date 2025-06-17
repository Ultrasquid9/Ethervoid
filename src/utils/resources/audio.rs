use macroquad::{prelude::warn, rand};
use tracing::error;

use kira::{
	AudioManager, AudioManagerSettings,
	sound::static_sound::{StaticSoundData, StaticSoundHandle},
};

use crate::{
	cores::audio::get_audio,
	utils::resources::{Global, global},
};

use super::{Resource, resource, set_resource};

/*
 *	Audio
 */

static MANAGER: Global<AudioManager> = global!(init_manager());
static SOUNDS: Resource<StaticSoundData> = resource();

/// Populates the Sounds `HashMap`
pub(super) fn create_sounds() {
	set_resource(&SOUNDS, get_audio());
}

/// Plays the sound at the provided key
pub fn play_sound(key: &str) -> Option<StaticSoundHandle> {
	let thing = SOUNDS.read();

	let Some(sound) = thing.get(key) else {
		error!("Sound {key} not found");
		return None;
	};

	match MANAGER.write().play(sound.clone()) {
		Ok(ok) => Some(ok),
		Err(e) => {
			error!("Error playing sound: {e}");
			None
		}
	}
}

/// Plays a random sound from the provided list of keys
pub fn play_random_sound(keys: &[impl AsRef<str>]) -> Option<StaticSoundHandle> {
	match keys {
		[] => {
			warn!("No neys provided!");
			None
		}
		keys => play_sound(keys[rand::gen_range(0, keys.len())].as_ref()),
	}
}

fn init_manager() -> AudioManager {
	match AudioManager::new(AudioManagerSettings::default()) {
		Ok(manager) => manager,
		Err(e) => {
			error!("Audio Manager could not be created: {e}");
			panic!()
		}
	}
}
