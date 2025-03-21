use crate::{cores::textures::get_textures, gameplay::draw::process::downscale};
use image::DynamicImage;
use imageproc::rgba_image;
use log::error;

use super::{Resource, resource};

/*
 *	Textures
 */

static TEXTURES: Resource<DynamicImage> = resource();

/// Populates the texture HashMap
pub(super) fn create_textures() {
	let mut access = TEXTURES.write();
	access.clear();
	*access = get_textures();
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> DynamicImage {
	let thing = TEXTURES.read();
	let Some(texture) = thing.get(key) else {
		error!("Texture {} not found", key);
		return downscale(
			&DynamicImage::ImageRgba8(rgba_image!(
				[0,0,0,255],[255,0,255,255];
				[255,0,255,255],[0,0,0,255]
			)),
			16,
		);
	};

	texture.clone()
}
