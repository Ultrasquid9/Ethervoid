use macroquad::prelude::*;
use raywoke::prelude::*;

use crate::{
	cores::map::Map,
	gameplay::ecs::sprite::{Rotation, Sprite},
	utils::resources::config::access_config,
};

pub async fn draw_map(map: &Map) {
	render_texture(&map.texture.clone(), dvec2(0., 0.), None).await;
}

/// Renders a texture based upon the screen scale
pub async fn render_texture(texture: &Texture2D, pos: DVec2, params: Option<DrawTextureParams>) {
	let screen_scale = access_config().screen_scale;

	let scale = DVec2::new(
		texture.width() as f64 * screen_scale,
		texture.height() as f64 * screen_scale,
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

pub async fn render_line(sprite: &mut Sprite) {
	let mut current = sprite.obj().pos;

	let pos = sprite.obj().pos;
	let target = sprite.obj().target;

	let jmp = access_config().screen_scale * (sprite.img().width() - 1) as f64;
	let screen_size = screen_width().max(screen_height()) as f64;

	if sprite.rotation() != Rotation::Angle {
		sprite.set_rotation(Rotation::Angle);
	}
	let (texture, _, _) = sprite.as_render_params();

	loop {
		render_texture(&texture, current, None).await;
		current = current.move_towards(target, jmp);

		if current.distance(pos) > screen_size || current.distance(target) < jmp {
			return;
		}
	}
}

/// Renders text
pub async fn render_text(string: &str, pos: DVec2, color: Color) {
	draw_text_ex(
		string,
		pos.x as f32,
		pos.y as f32,
		TextParams {
			font_size: (access_config().screen_scale * 12.) as u16,
			color,

			..Default::default()
		},
	);
}

pub fn pixel_offset(base: f64) -> f32 {
	let screen_scale = access_config().screen_scale;

	((base / screen_scale).round() * screen_scale) as f32
}

/// Draws a Barrier
/// Probably temporary, may remain for debug
pub fn draw_bar(bar: &Barrier) {
	draw_line(
		bar.0.x() as f32,
		bar.0.y() as f32,
		bar.1.x() as f32,
		bar.1.y() as f32,
		6.,
		BLUE,
	);
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
