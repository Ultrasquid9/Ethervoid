use raylite::Barrier;
use macroquad::prelude::*;

use super::{
	SCREEN_SCALE,
	to_texture
};

use crate::{
	cores::map::Map, 
	menu::FONT, 
	utils::camera_scale
};

pub async fn draw_map(map: &Map) {
	render_texture(
		&to_texture(map.texture.clone()), 
		vec2(0., 0.), 
		None
	).await;
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

/// Renders text 
pub async fn render_text(string: &str, pos: Vec2, color: Color) {
	draw_text_ex(string, pos.x, pos.y, TextParams {
		font: Some(&FONT),
		font_size: camera_scale() as u16 / 12,
		color,

		..Default::default()
	});
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

pub fn darken_screen() {
	draw_rectangle(
		0., 
		0., 
		screen_width(), 
		screen_height(), 
		Color::new(0., 0., 0., 0.25)
	);
}