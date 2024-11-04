use image::{imageops::resize, DynamicImage};
use macroquad::texture::Texture2D;

pub fn downscale(img: &DynamicImage, size: u32) -> Texture2D {
	let image = resize(
		img, 
		size, 
		size, 
		image::imageops::FilterType::Nearest
	);

	return to_texture(DynamicImage::ImageRgba8(image));
}

pub fn to_texture(img: DynamicImage) -> Texture2D {
	let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_bytes());
	texture.set_filter(macroquad::texture::FilterMode::Nearest);
	return texture
}