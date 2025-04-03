use std::sync::OnceLock;

use crate::{cores::textures::get_textures, gameplay::draw::process::downscale};
use image::DynamicImage;
use imageproc::rgba_image;
use log::error;

use super::{Resource, get_resource_ref, resource};

/*
 *	Textures
 */

static ERR_TEXTURE: OnceLock<DynamicImage> = OnceLock::new();
static TEXTURES: Resource<DynamicImage> = resource();

/// Populates the texture HashMap
pub(super) fn create_textures() {
	let mut access = TEXTURES.write();
	access.clear();
	*access = get_textures();
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> &DynamicImage {
	if let Some(texture) = get_resource_ref(&TEXTURES, key) {
		texture
	} else {
		error!("Texture {key} not found");

		ERR_TEXTURE.get_or_init(|| {
			downscale(
				&DynamicImage::ImageRgba8(rgba_image!(
					[0,0,0,255],[255,0,255,255];
					[255,0,255,255],[0,0,0,255]
				)),
				16,
			)
		})
	}
}
