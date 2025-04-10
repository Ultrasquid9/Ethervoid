use std::sync::LazyLock;

use crate::{cores::textures::get_textures, gameplay::draw::process::downscale};
use image::DynamicImage;
use imageproc::rgba_image;
use log::error;

use super::{Resource, get_resource_ref, resource, set_resource};

/*
 *	Textures
 */

static ERR_TEXTURE: LazyLock<DynamicImage> = LazyLock::new(init_err_texture);
static TEXTURES: Resource<DynamicImage> = resource();

/// Populates the texture HashMap
pub(super) fn create_textures() {
	set_resource(&TEXTURES, get_textures());
}

/// Gets the image at the provided key
pub fn access_image(key: &str) -> &DynamicImage {
	if let Some(texture) = get_resource_ref(&TEXTURES, key) {
		texture
	} else {
		error!("Texture {key} not found");
		&ERR_TEXTURE
	}
}

fn init_err_texture() -> DynamicImage {
	downscale(
		&DynamicImage::ImageRgba8(rgba_image!(
			[0,0,0,255],[255,0,255,255];
			[255,0,255,255],[0,0,0,255]
		)),
		16,
	)
}
