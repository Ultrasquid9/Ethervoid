use image::{DynamicImage, Rgba};
use imageproc::geometric_transformations::rotate_about_center;
use macroquad::{math::{Rect, Vec2}, texture::DrawTextureParams};

use crate::{gameplay::draw::{process::{downscale, to_texture}, render::render_texture, SCREEN_SCALE}, utils::{get_delta_time, resources::textures::access_image}};

use super::obj::{Axis, Obj};

#[derive(Clone)]
pub struct Sprite {
	pub sprite: DynamicImage,
	pub obj: Obj,

	rotation: Rotation,
	frames: Frames,
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Rotation {
	Angle,
	EightWay,
	Static
}

#[derive(Clone)]
pub struct Frames {
	frame_order: Vec<u32>,
	frame_time: f32,

	anim_time: f32,
	anim_completed: bool
}

impl Sprite {
	pub fn new(
		obj: Obj, 
		_size: u32, 
		key: &str, 
		rotation: Rotation, 
		frames: Frames
	) -> Self {
		Self {
			sprite: if rotation == Rotation::Angle {
				downscale(
					access_image(key),
					obj.size as u32
				)
			} else { access_image(key) },
			obj,

			rotation,
			frames,
		}
	}

	pub fn update(&mut self, new_obj: Obj) {
		if self.obj.pos == new_obj.pos 
		&& self.rotation == Rotation::EightWay {
			self.frames.reset();
		} else {
			self.frames.update(); 
			self.obj = new_obj;
		}
	}

	pub fn anim_completed(&self) -> bool {
		self.frames.anim_completed
	}

	pub async fn render(&self) {
		let size = if self.rotation == Rotation::EightWay {
			self.sprite.height() / 5
		} else {
			self.sprite.height()
		};

		let x_pos = self.frames.get_frame() * size;

		let y_pos: u32 = if self.rotation != Rotation::EightWay {
			0
		} else {
			// There is definitely a far better way to do this
			// I apologize to whoever has to deal with this in the future 
			if self.obj.axis_horizontal != Axis::None && self.obj.axis_vertical != Axis::None {
				if self.obj.axis_vertical == Axis::Positive {
					size       // Diagonal up
				} else {
					size * 3   // Diagonal down
				}
			} else if self.obj.axis_horizontal != Axis::None {
				size * 2       // Left/right
			} else if self.obj.axis_vertical == Axis::Positive {
				0              // Down
			} else {
				size * 4       // Up
			}
		};

		render_texture(
			&to_texture(if self.rotation == Rotation::Angle {
				DynamicImage::ImageRgba8(rotate_about_center(
					self.sprite.crop_imm(
						x_pos, 
						y_pos, 
						size, 
						size
					).as_rgba8().unwrap(), 
					(self.obj.target.y - self.obj.pos.y).atan2(self.obj.target.x - self.obj.pos.x),
					imageproc::geometric_transformations::Interpolation::Nearest, 
					Rgba([0, 0, 0, 0])
				))
			} else {
				self.sprite.clone()
			}),
			Vec2::new(
				self.obj.pos.x + if self.rotation == Rotation::EightWay { self.sprite.width() as f32 } else { 0. }, 
				self.obj.pos.y + if self.rotation == Rotation::EightWay { self.sprite.height() as f32 } else { 0. }
			), 
			Some(DrawTextureParams {
				source: if self.rotation == Rotation::EightWay {
					Some(
						Rect::new(
							x_pos as f32,
							y_pos as f32,
							size as f32,
							size as f32
						)
					)
				} else { None },
				flip_x: self.obj.axis_horizontal == Axis::Negative && self.rotation == Rotation::EightWay,
				dest_size: Some(Vec2::new(size as f32 * SCREEN_SCALE, size as f32 * SCREEN_SCALE)),
				..Default::default()
			})
		).await;
	}
}

impl Frames {
	pub fn new_entity() -> Self {
		Self {
			frame_order: vec![0, 1, 0, 2],
			frame_time: 16.,

			anim_time: 0.,
			anim_completed: false
		}
	}

	pub fn new_attack() -> Self {
		Self {
			frame_order: vec![0, 1, 2],
			frame_time: 4.,

			anim_time: 0.,
			anim_completed: false
		}
	}

	pub fn new_static() -> Self {
		Self {
			frame_order: vec![0],
			frame_time: 0.,

			anim_time: 0.,
			anim_completed: false
		}
	}

	fn update(&mut self) {
		self.anim_time += get_delta_time();

		if self.anim_time as usize >= (self.frame_order.len() * self.frame_time as usize) {
			self.anim_time = 0.;
			self.anim_completed = true;
		}
	}

	fn reset(&mut self) {
		self.anim_time = 0.;
	}

	fn get_frame(&self) -> u32 {
		*self.frame_order.get((self.anim_time / self.frame_time) as usize).unwrap()
	}
}
