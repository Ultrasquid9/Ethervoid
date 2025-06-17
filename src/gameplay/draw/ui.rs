use image::{DynamicImage, GenericImageView};
use macroquad::prelude::*;

use crate::{
	gameplay::{draw::process::to_texture, ecs::health::Health},
	menu::average_screen_size,
	utils::resources::textures::access_image,
};

pub struct PlayerUi {
	hp_texture: Texture2D,
	hp_bar_texture: Texture2D,
	hp_bar_offset: Vec2,

	temp_texture: Texture2D,
	temp_bar_texture: Texture2D,
	temp_bar_offset: Vec2,
}

impl PlayerUi {
	pub fn new(
		hp_texture: &str,
		hp_bar_texture: &str,
		temp_texture: &str,
		temp_bar_texture: &str,
	) -> Self {
		let hp_texture = to_texture(access_image(hp_texture));
		let (hp_bar_offset, hp_bar_img) = remove_alpha(access_image(hp_bar_texture));

		let temp_texture = to_texture(access_image(temp_texture));
		let (temp_bar_offset, temp_bar_img) = remove_alpha(access_image(temp_bar_texture));

		Self {
			hp_texture,
			hp_bar_texture: to_texture(&hp_bar_img),
			hp_bar_offset,

			temp_texture,
			temp_bar_texture: to_texture(&temp_bar_img),
			temp_bar_offset,
		}
	}

	pub fn draw_hp(&self, health: &Health) {
		let scale = average_screen_size() / 300.;
		let size = vec2(self.hp_texture.width(), self.hp_texture.height()) * scale;

		draw_texture_ex2(
			&self.hp_texture,
			Vec2::ZERO,
			DrawTextureParams {
				dest_size: Some(size),
				..Default::default()
			},
		);

		draw_bar_right(
			&self.hp_bar_texture,
			self.hp_bar_offset,
			Vec2::ZERO,
			scale,
			health.max,
			health.hp,
		);
	}

	pub fn draw_temp(&self, temp: f64) {
		let scale = average_screen_size() / 300.;
		let pos = vec2(screen_width() - (self.temp_texture.width() * scale), 0.);
		let size = vec2(self.hp_texture.width(), self.hp_texture.height()) * scale;

		draw_texture_ex2(
			&self.temp_texture,
			pos,
			DrawTextureParams {
				dest_size: Some(size),
				..Default::default()
			},
		);

		draw_bar_left(
			&self.temp_bar_texture,
			self.temp_bar_offset,
			pos,
			scale,
			100.,
			temp,
		);
	}
}

fn draw_bar_right(texture: &Texture2D, offset: Vec2, pos: Vec2, scale: f32, max: f64, current: f64) {
	let mut size = vec2(texture.width(), texture.height());
	size.x = (size.x / max as f32) * current as f32;
	size = size.round() * scale;

	draw_texture_ex2(
		texture,
		calc_pos(pos, offset, size, scale, texture.width()),
		DrawTextureParams {
			dest_size: Some(size),
			source: Some(Rect::new(0., 0., size.x / scale, texture.height())),

			..Default::default()
		},
	);
}

fn draw_bar_left(texture: &Texture2D, offset: Vec2, pos: Vec2, scale: f32, max: f64, current: f64) {
	let mut size = vec2(texture.width(), texture.height());
	size.x = (size.x / max as f32) * current as f32;
	size = size.round() * scale;

	draw_texture_ex2(
		texture,
		calc_pos(pos, offset, size, scale, texture.width()),
		DrawTextureParams {
			dest_size: Some(size),
			source: Some(Rect::new(texture.width()-(size.x / scale), 0., size.x / scale, texture.height())),

			..Default::default()
		},
	);
}

fn calc_pos(pos: Vec2, offset: Vec2, size: Vec2, scale: f32, width: f32) -> Vec2 {
	(offset * scale) + pos.with_x(pos.x - size.x + (width * scale))
}

fn remove_alpha(img: &DynamicImage) -> (Vec2, DynamicImage) {
	let mut x_largest = 0;
	let mut x_smallest = img.width();
	let mut y_largest = 0;
	let mut y_smallest = img.height();

	for (width, height, color) in img.pixels() {
		if *color.0.last().expect("Slice is not empty") == 0 {
			continue;
		}

		if width > x_largest {
			x_largest = width;
		}
		if width < x_smallest {
			x_smallest = width;
		}

		if height > y_largest {
			y_largest = height;
		}
		if height < y_smallest {
			y_smallest = height;
		}
	}

	if x_smallest > x_largest || y_smallest > y_largest {
		return (Vec2::ZERO, img.crop_imm(0, 0, 0, 0));
	}

	(
		vec2(x_smallest as f32, y_smallest as f32),
		img.crop_imm(
			x_smallest,
			y_smallest,
			x_largest - x_smallest + 1,
			y_largest - y_smallest + 1,
		),
	)
}

fn draw_texture_ex2(texture: &Texture2D, pos: Vec2, params: DrawTextureParams) {
	draw_texture_ex(texture, pos.x, pos.y, WHITE, params);
}
