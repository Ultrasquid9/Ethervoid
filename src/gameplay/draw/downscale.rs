use imageproc::{geometric_transformations::rotate_about_center, image::{imageops::{resize, FilterType::Nearest}, DynamicImage, Rgba}};
use macroquad::texture::Texture2D;

/// Downscales and rotates the provided image
pub fn downscale(img: &DynamicImage, size: u32, rotation: f32) -> Texture2D {
	let image = resize(
		img, 
		size, 
		size, 
		Nearest
	);

	let image = rotate_about_center(
		&image, 
		rotation, 
		imageproc::geometric_transformations::Interpolation::Nearest, 
		Rgba {
			0: [0, 0, 0, 0] // Clear
		}
	);

	return to_texture(DynamicImage::ImageRgba8(image));
}

/// Transforms a `DynamicImage` into a `Texture2D`
pub fn to_texture(img: DynamicImage) -> Texture2D {
	let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_bytes());
	texture.set_filter(macroquad::texture::FilterMode::Nearest);
	return texture
}
