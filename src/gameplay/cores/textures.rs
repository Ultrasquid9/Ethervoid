use std::{collections::HashMap, thread};

use image::{DynamicImage, ImageReader};

use super::{gen_name, get_files};

/// Provides a HashMap containing all Textures
pub fn get_textures() -> HashMap<String, DynamicImage> {
	let mut textures: HashMap<String, DynamicImage> = HashMap::new();

	let mut texture_handles = Vec::new();
	let mut names = Vec::new();

	for i in get_files(String::from("sprites")) {
		names.push(gen_name(&i));

		texture_handles.push(thread::spawn(move || -> DynamicImage {
			ImageReader::open(&i)
				.unwrap()
				.decode()
				.unwrap()
		}));
	}

	for i in texture_handles.into_iter().enumerate() {
		println!("{}", names[i.0]);
		textures.insert(names[i.0].clone(), i.1.join().unwrap());
	}

	return textures;
}
