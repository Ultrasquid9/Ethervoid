use std::fs;

use macroquad::{color::WHITE, math::Vec2, texture::{draw_texture_ex, DrawTextureParams, FilterMode, Texture2D}, window::{screen_height, screen_width}};

use super::SCREEN_SCALE;

pub fn draw_tilemap(texture: &Texture2D) {
	for y in (0..screen_height() as isize).step_by(16 * SCREEN_SCALE as usize) {
		for x in (0..screen_width() as isize).step_by(16 * SCREEN_SCALE as usize) {
			render_texture(&texture, Vec2::new(x as f32, y as f32), None);
		}
	}
}

/// Loads a picture from the provided directory (NOTE: WIP)
pub fn load_texture(dir: &str) -> Texture2D {
	let texture = Texture2D::from_file_with_format(
		fs::read(dir).unwrap().as_slice(), 
		None
	);
	texture.set_filter(FilterMode::Nearest);
	return texture;
}

/// Renders a texture based upon the screen scale
pub fn render_texture(texture: &Texture2D, pos: Vec2, params: Option<DrawTextureParams>) {
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
