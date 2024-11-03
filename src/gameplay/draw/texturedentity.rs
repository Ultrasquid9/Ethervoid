use macroquad::{math::{Rect, Vec2}, texture::{DrawTextureParams, Texture2D}};

use crate::gameplay::player::Axis;

use super::{textures::render_texture, SCREEN_SCALE};

pub trait TexturedEntity {
	fn update_texture(&mut self);
}

pub struct Texture {
	pub sprite: Texture2D,

	pos: Vec2,

	pub moving: bool,
	anim_time: u8,

	dir_horizontal: Axis,
	dir_vertical: Axis
}

impl Texture {
	pub fn new(sprite: Texture2D) -> Self {
		Self {
			sprite,

			pos: Vec2::new(0., 0.),

			moving: false,
			anim_time: 0,

			dir_horizontal: Axis::None,
			dir_vertical: Axis::None
		}
	}

	/// Updates the texture with the provided texture data
	pub fn update(&mut self, pos: Vec2, dir_horizontal: Axis, dir_vertical: Axis, moving: bool) {
		self.pos = pos;
		self.moving = moving;

		if self.moving {
			self.dir_horizontal = dir_horizontal;
			self.dir_vertical = dir_vertical;
		}

		if self.moving && self.anim_time < 64 {
			self.anim_time += 1;
		} else {
			self.anim_time = 0;
		}
	}

	/// Renders the texture with the current texture data
	pub fn render(&self) {
		let size = self.sprite.height() / 5.;

		// There is definitely a far better way to do this
		// I apologize to whoever has to deal with this in the future 
		let y_pos = if self.dir_horizontal != Axis::None && self.dir_vertical != Axis::None {
			if self.dir_vertical == Axis::Positive {
				size       // Diagonal up
			} else {
				size * 3.  // Diagonal down
			}
		} else if self.dir_horizontal != Axis::None {
			size * 2.      // Left/right
		} else {
			if self.dir_vertical == Axis::Positive {
				0.         // Down
			} else {
				size * 4.  // Up
			}
		};

		println!("{}", self.anim_time / 16);
		let x_pos = match self.anim_time / 16 {
			1 => size,
			3 => size * 2.,
			_ => 0.
		};

		render_texture(
			&self.sprite, 
			Vec2::new(
				self.pos.x + self.sprite.width(), 
				self.pos.y + self.sprite.height()
			), 
			Some(DrawTextureParams {
				source: Some(
					Rect::new(
						x_pos,
						y_pos,
						size,
						size
					)
				),
				flip_x: if self.dir_horizontal == Axis::Negative {
					true
				} else {
					false
				},
				dest_size: Some(Vec2::new(size * SCREEN_SCALE, size * SCREEN_SCALE)),
				..Default::default()
			})
		);
	}
}
