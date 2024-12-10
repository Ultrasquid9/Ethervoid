use std::sync::mpsc;
use ahash::HashMap;

use imageproc::image::{
	ColorType, 
	DynamicImage, 
	ImageReader
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::{
	gen_name, 
	get_files
};

/// Provides a HashMap containing all Textures
pub fn get_textures() -> HashMap<String, DynamicImage> {
	let mut textures: HashMap<String, DynamicImage> = HashMap::default();

	let (transciever, receiver) = mpsc::channel();

	get_files("sprites".to_string())
		.par_iter()
		.for_each(|dir| {
			let img = ImageReader::open(dir)
				.unwrap()
				.decode();

			let Ok(img) = img 
			else {
				println!("{}", img.err().unwrap());
				return
			};

			let _ = transciever.send((
				gen_name(dir), 
				if img.color() == ColorType::Rgba8 {
					img
				} else {
					DynamicImage::ImageRgba8(img.to_rgba8())
				}
			));
		});

	drop(transciever);

	for (name, image) in receiver {
		textures.insert(name, image);
	}

	textures
}
