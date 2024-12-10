use std::sync::mpsc;
use ahash::HashMap;
use kira::sound::static_sound::StaticSoundData;
use rayon::prelude::*;

use super::{
	gen_name, 
	get_files
};

/// Provides a HashMap containing all Textures
pub fn get_audio() -> HashMap<String, StaticSoundData> {
	let mut audio: HashMap<String, StaticSoundData> = HashMap::default();

	let (transciever, receiver) = mpsc::channel();

	get_files("audio".to_string())
		.par_iter()
		.for_each(|dir| {
			let Ok(sound) = StaticSoundData::from_file(dir)
			else { return };

			let _ = transciever.send((gen_name(dir), sound));
		});

	drop(transciever);

	for (name, sound) in receiver {
		audio.insert(name, sound);
	}

	audio
}
