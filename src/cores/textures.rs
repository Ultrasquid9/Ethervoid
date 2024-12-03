use std::thread;
use ahash::HashMap;

use imageproc::image::{
	ColorType, 
	DynamicImage, 
	ImageReader
};

use super::{
	gen_name, 
	get_files
};

/// Provides a HashMap containing all Textures
pub fn get_textures() -> HashMap<String, DynamicImage> {
	let mut textures: HashMap<String, DynamicImage> = HashMap::default();

	let mut texture_handles = Vec::new();
	let mut names = Vec::new();

	for i in get_files(String::from("sprites")) {
		names.push(gen_name(&i));

		texture_handles.push(thread::spawn(move || -> DynamicImage {
			let img = ImageReader::open(&i)
				.unwrap()
				.decode()
				.unwrap();

			if img.color() == ColorType::Rgba8 {
				img
			} else {
				DynamicImage::ImageRgba8(img.to_rgba8())
			}
		}));
	}

	for i in texture_handles.into_iter().enumerate() {
		textures.insert(names[i.0].clone(), i.1.join().unwrap());
	}

	textures
}
