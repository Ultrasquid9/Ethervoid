use macroquad::prelude::*;
use raywoke::Barrier;

use super::{SCREEN_SCALE, to_texture};

use crate::{cores::map::Map, utils::camera_scale};

pub async fn draw_map(map: &Map) {
	render_texture(&to_texture(map.texture.clone()), dvec2(0., 0.), None).await;
}

/// Renders a texture based upon the screen scale
pub async fn render_texture(texture: &Texture2D, pos: DVec2, params: Option<DrawTextureParams>) {
	let scale = DVec2::new(
		texture.width() as f64 * SCREEN_SCALE,
		texture.height() as f64 * SCREEN_SCALE,
	);

	draw_texture_ex(
		texture,
		pixel_offset(pos.x - scale.x / 2.),
		pixel_offset(pos.y - scale.y / 2.),
		WHITE,
		match params {
			Option::Some(params) => params,
			Option::None => DrawTextureParams {
				dest_size: Some(scale.as_vec2()),
				..Default::default()
			},
		},
	);
}

/// Renders text
pub async fn render_text(string: &str, pos: DVec2, color: Color) {
	draw_text_ex(
		string,
		pos.x as f32,
		pos.y as f32,
		TextParams {
			font_size: camera_scale() as u16 / 12,
			color,

			..Default::default()
		},
	);
}

pub fn pixel_offset(base: f64) -> f32 {
	((base / SCREEN_SCALE).round() * SCREEN_SCALE) as f32
}

/// Draws a Barrier
/// Probably temporary, may remain for debug
pub fn draw_bar(bar: &Barrier) {
	draw_line(bar.0.x(), bar.0.y(), bar.1.x(), bar.1.y(), 6., BLUE);
}

pub fn darken_screen() {
	draw_rectangle(
		0.,
		0.,
		screen_width(),
		screen_height(),
		Color::new(0., 0., 0., 0.25),
	);
}
