use macroquad::{math::Vec2, texture::{DrawTextureParams, Texture2D}};

use crate::gameplay::player::Axis;

use super::textures::render_texture;

pub trait TexturedEntity {
	fn update_texture(&mut self);
}

pub struct Texture {
	pub sprite: Texture2D,

	pos: Vec2,
	moving: bool,
	dir_horizontal: Axis,
	dir_vertical: Axis
}

impl Texture {
	pub fn new(sprite: Texture2D) -> Self {
		Self {
			sprite,
			pos: Vec2::new(0., 0.),
			moving: false,
			dir_horizontal: Axis::None,
			dir_vertical: Axis::None
		}
	}

	/// Updates the texture with the provided texture data
	pub fn update(&mut self, pos: Vec2, dir_horizontal: Axis, dir_vertical: Axis, moving: bool) {
		self.pos = pos;
		self.dir_horizontal = dir_horizontal;
		self.dir_vertical = dir_vertical;
		self.moving = moving;
	}

	pub fn update_axis(&mut self, old_pos: &Vec2, new_pos: &Vec2) {
		if old_pos.x > new_pos.x {
			self.dir_horizontal = Axis::Positive
		} else if old_pos.x < new_pos.x {
			self.dir_horizontal = Axis::Negative
		} else {
			self.dir_horizontal = Axis::None
		}

		if old_pos.y > new_pos.y {
			self.dir_horizontal = Axis::Positive
		} else if old_pos.y < new_pos.y {
			self.dir_horizontal = Axis::Negative
		} else {
			self.dir_horizontal = Axis::None
		}	
	}

	/// Renders the texture with the current texture data
	pub fn render(&self) {
		render_texture(
			&self.sprite, 
			self.pos, 
			Some(DrawTextureParams {
				..Default::default()
			})
		);
	}
}

