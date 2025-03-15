use crate::prelude::*;
use kira::sound::static_sound::StaticSoundData;
use std::sync::mpsc;

use super::{gen_name, get_files};

/// Provides a HashMap containing all Textures
pub fn get_audio() -> HashMap<String, StaticSoundData> {
	let mut audio: HashMap<String, StaticSoundData> = HashMap::default();

	let (transciever, receiver) = mpsc::channel();

	get_files("audio".to_string()).par_iter().for_each(|dir| {
		let name: String = gen_name(dir);
		let sound = StaticSoundData::from_file(dir);

		let Ok(sound) = sound else {
			warn!("Audio {} failed to load: {}", name, sound.err().unwrap());
			return;
		};

		info!("Audio {} loaded!", name);

		let _ = transciever.send((name, sound));
	});

	drop(transciever);

	for (name, sound) in receiver {
		audio.insert(name, sound);
	}

	audio
}
