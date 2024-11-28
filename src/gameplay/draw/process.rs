use imageproc::image::DynamicImage;

use fast_image_resize::{
	ResizeOptions, 
	Resizer
};

use macroquad::texture::Texture2D;


/// Downscales the provided image
pub fn downscale(img: DynamicImage, size: u32) -> DynamicImage {
	let smallest_side = if img.width() < img.height() {
		img.width()
	} else {
		img.height()
	};

	let mut downscaled_img = DynamicImage::new_rgba8(
		(img.width() / smallest_side) * size, 
		(img.height() / smallest_side) * size
	);

	let mut resizer =  Resizer::new();
	resizer.resize(
		&img, 
		&mut downscaled_img, 
		&Some(ResizeOptions {
			algorithm: fast_image_resize::ResizeAlg::Nearest,
			..Default::default()
		})
	).unwrap();

	downscaled_img
}

/// Transforms a `DynamicImage` into a `Texture2D`
pub fn to_texture(img: DynamicImage) -> Texture2D {
	let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_bytes());
	texture.set_filter(macroquad::texture::FilterMode::Nearest);
	texture
}