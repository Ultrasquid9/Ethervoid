use futures::future::join_all;
use raylite::Barrier;
use super::SCREEN_SCALE;

use macroquad::{
	color::{BLUE, WHITE}, math::Vec2, shapes::draw_line, texture::{
		draw_texture_ex, 
		DrawTextureParams, 
		Texture2D
	}, window::{
		screen_height, 
		screen_width
	}
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
		texture, 
		pixel_offset(pos.x - scale.x / 2.),
		pixel_offset(pos.y - scale.y / 2.),
		WHITE, 
		match params {
			Option::Some(params) => params,
			Option::None => DrawTextureParams {
				dest_size: Some(scale),
				..Default::default()
			}
		}
	);
}

pub fn pixel_offset(base: f32) -> f32 {
	(base / SCREEN_SCALE).round() * SCREEN_SCALE
}

/// Draws a Barrier
/// Probably temporary, may remain for debug
pub fn draw_bar(bar: &Barrier) {
	draw_line(
		bar.positions.0.0, 
		bar.positions.0.1, 
		bar.positions.1.0, 
		bar.positions.1.1, 
		6., 
		BLUE
	);
}