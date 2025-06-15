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
}

impl PlayerUi {
	pub fn new(
		hp_texture: &str,
		hp_bar_texture: &str,
		_heat_texture: &str,
		_heat_bar_texture: &str,
	) -> Self {
		let hp_texture = to_texture(access_image(hp_texture));
		let (hp_bar_offset, hp_bar_img) = remove_alpha_from_img(access_image(hp_bar_texture));

		Self {
			hp_texture,
			hp_bar_texture: to_texture(&hp_bar_img),
			hp_bar_offset,
		}
	}

	pub fn draw_hp(&self, health: &Health) {
		let scale = average_screen_size() / 300.;
		let size = vec2(self.hp_texture.width(), self.hp_texture.height()) * scale;

		draw_texture_ex(
			&self.hp_texture,
			0.,
			0.,
			WHITE,
			DrawTextureParams {
				dest_size: Some(size),
				..Default::default()
			},
		);

		let mut size = vec2(self.hp_bar_texture.width(), self.hp_bar_texture.height()) * scale;
		size.x = (size.x / health.max as f32) * health.hp as f32;

		draw_texture_ex(
			&self.hp_bar_texture,
			self.hp_bar_offset.x * scale,
			self.hp_bar_offset.y * scale,
			WHITE,
			DrawTextureParams {
				dest_size: Some(size),
				..Default::default()
			},
		);
	}
}

fn remove_alpha_from_img(img: &DynamicImage) -> (Vec2, DynamicImage) {
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
