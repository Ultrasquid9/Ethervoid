use crate::prelude::*;
use kira::sound::static_sound::StaticSoundData;
use std::sync::mpsc;

use super::{gen_name, get_files};

/// Provides a HashMap containing all Textures
pub fn get_audio() -> HashMap<String, StaticSoundData> {
	let mut audio: HashMap<String, StaticSoundData> = HashMap::default();

	let (transciever, receiver) = mpsc::channel();

	for dir in get_files("audio") {
		let name: String = gen_name(&dir);
		let sound = StaticSoundData::from_file(dir);

		let sound = match sound {
			Ok(sound) => sound,
			Err(e) => {
				warn!("Audio {name} failed to load: {e}");
				continue;
			}
		};

		info!("Audio {name} loaded!");

		_ = transciever.send((name, sound));
	}

	drop(transciever);

	for (name, sound) in receiver {
		audio.insert(name, sound);
	}

	audio
}
