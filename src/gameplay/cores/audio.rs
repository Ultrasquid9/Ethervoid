use std::thread;
use ahash::HashMap;
use kira::sound::static_sound::StaticSoundData;

use super::{
	gen_name, 
	get_files
};

/// Provides a HashMap containing all Textures
pub fn get_audio() -> HashMap<String, StaticSoundData> {
	let mut audio: HashMap<String, StaticSoundData> = HashMap::default();

	let mut audio_handles = Vec::new();
	let mut names = Vec::new();

	for i in get_files(String::from("audio")) {
		names.push(gen_name(&i));

		audio_handles.push(thread::spawn(move || -> StaticSoundData {
			StaticSoundData::from_file(i).unwrap()
		}));
	}

	for i in audio_handles.into_iter().enumerate() {
		audio.insert(names[i.0].clone(), i.1.join().unwrap());
	}

	return audio;
}
