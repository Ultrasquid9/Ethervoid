use futures::future::join_all;
use imageproc::image::DynamicImage;
use super::SCREEN_SCALE;

use fast_image_resize::{
	ResizeOptions, 
	Resizer
};

use macroquad::{
	texture::{
		draw_texture_ex, 
		DrawTextureParams, 
		Texture2D
	}, 
	window::{
		screen_height, 
		screen_width
	},
	color::WHITE, 
	math::Vec2
};


pub async fn draw_tilemap(texture: Texture2D) {
	let mut futures = Vec::new();

	for y in (0..screen_height() as isize).step_by(16 * SCREEN_SCALE as usize) {
		for x in (0..screen_width() as isize).step_by(16 * SCREEN_SCALE as usize) {
			futures.push(render_texture(&texture, Vec2::new(x as f32, y as f32), None));
		}
	}

	join_all(futures).await;
}

/// Renders a texture based upon the screen scale
pub async fn render_texture(texture: &Texture2D, pos: Vec2, params: Option<DrawTextureParams>) {
	let scale = Vec2::new(
		texture.width() * SCREEN_SCALE, 
		texture.height() * SCREEN_SCALE
	);

	draw_texture_ex(
		&texture, 
		pixel_offset(pos.x - scale.x / 2.),
		pixel_offset(pos.y - scale.y / 2.),
		WHITE, 
		match params {
			Some(params) => params,
			None => DrawTextureParams {
				dest_size: Some(scale),
				..Default::default()
			}
		}
	);
}

pub fn pixel_offset(base: f32) -> f32 {
	return (base / SCREEN_SCALE).round() * SCREEN_SCALE;
}

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

	return downscaled_img;
}

/// Transforms a `DynamicImage` into a `Texture2D`
pub fn to_texture(img: DynamicImage) -> Texture2D {
	let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_bytes());
	texture.set_filter(macroquad::texture::FilterMode::Nearest);
	return texture
}
