use ahash::HashMap;

use crate::{
	gameplay::draw::{SCREEN_SCALE, process::downscale},
	utils::{error::EtherVoidError, get_delta_time, resources::textures::access_image},
};

use macroquad::{
	math::{DVec2, Rect, Vec2},
	texture::DrawTextureParams,
};

use imageproc::{
	geometric_transformations::rotate_about_center,
	image::{DynamicImage, Rgba},
};

use serde::{Deserialize, Serialize};

use super::obj::{Axis, Obj};

#[derive(Clone)]
pub struct Sprite {
	sprite: DynamicImage,
	obj: Obj,

	cache: Option<(u32, DynamicImage)>,

	rotation: Rotation,
	frames: Frames,

	shaking: f64,

	current_anim: Option<String>,
	anims: HashMap<String, Frames>,
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Rotation {
	Angle,
	EightWay,
	Static,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Frames {
	frame_order: Vec<u32>,
	frame_time: f64,

	#[serde(skip)]
	anim_time: f64,
	#[serde(skip)]
	anim_completed: bool,
}

impl Sprite {
	pub fn new(
		obj: Obj,
		key: &str,
		rotation: Rotation,
		frames: Frames,
		anims: HashMap<String, Frames>,
	) -> Self {
		Self {
			sprite: if rotation == Rotation::Angle {
				downscale(access_image(key), obj.size as u32)
			} else {
				access_image(key)
			},
			obj,

			cache: None,

			rotation,
			frames,

			shaking: 0.,

			current_anim: None,
			anims,
		}
	}

	pub fn update(&mut self, new_obj: Obj) {
		if self.shaking > 0. {
			self.shaking -= get_delta_time();
		}

		if self.current_anim.is_some() {
			let anim = self
				.anims
				.get_mut(self.current_anim.as_ref().unwrap())
				.unwrap();

			anim.update();

			if self.rotation != Rotation::EightWay {
				self.obj = new_obj;
			}
			return;
		}

		if self.obj.pos == new_obj.pos && self.rotation == Rotation::EightWay {
			self.frames.reset();
		} else {
			self.frames.update();
			self.obj = new_obj;
		}
	}

	pub fn obj(&self) -> Obj {
		self.obj
	}

	pub fn shake(&mut self) {
		self.shaking = 20.;
	}

	pub fn anim_completed(&self) -> bool {
		self.frames.anim_completed
	}

	pub fn set_new_anim(&mut self, key: String) -> Result<(), Box<EtherVoidError>> {
		if !self.anims.contains_key(&key) {
			return Err(Box::new(EtherVoidError::AnimNotFound(key)));
		}

		self.current_anim = Some(key);

		self.frames.reset();
		for (_, anim) in self.anims.iter_mut() {
			anim.reset();
		}

		Ok(())
	}

	pub fn set_default_anim(&mut self) {
		self.current_anim = None
	}

	pub async fn to_render_params(&mut self) -> (DynamicImage, DVec2, Option<DrawTextureParams>) {
		let size = if self.rotation == Rotation::EightWay {
			self.sprite.height() / 5
		} else {
			self.sprite.height()
		};

		let mut x_pos = if self.current_anim.is_some() {
			self.anims
				.get(self.current_anim.as_ref().unwrap())
				.unwrap()
				.get_frame()
		} else {
			self.frames.get_frame()
		} * size;

		let mut y_pos = if self.rotation != Rotation::EightWay {
			0
		} else {
			// There is definitely a far better way to do this
			// I apologize to whoever has to deal with this in the future
			if self.obj.axis_horizontal != Axis::None && self.obj.axis_vertical != Axis::None {
				if self.obj.axis_vertical == Axis::Positive {
					size // Diagonal up
				} else {
					size * 3 // Diagonal down
				}
			} else if self.obj.axis_horizontal != Axis::None {
				size * 2 // Left/right
			} else if self.obj.axis_vertical == Axis::Positive {
				0 // Down
			} else {
				size * 4 // Up
			}
		};

		if self.shaking > 0. {
			x_pos += (self.shaking.sin() * 3.) as u32;
		}

		x_pos = x_pos.clamp(0, self.sprite.width() - 1);
		y_pos = y_pos.clamp(0, self.sprite.height() - 1);

		(
			if self.rotation == Rotation::Angle {
				self.texture_angle(x_pos, y_pos)
			} else {
				self.sprite.clone()
			},
			DVec2::new(
				self.obj.pos.x
					+ match self.rotation {
						Rotation::Angle => 0.,
						Rotation::Static => self.sprite.width() as f64 / 2.,
						_ => self.sprite.width() as f64,
					},
				self.obj.pos.y
					+ match self.rotation {
						Rotation::Angle => 0.,
						Rotation::Static => self.sprite.width() as f64 / 2.,
						_ => self.sprite.height() as f64,
					},
			),
			Some(DrawTextureParams {
				source: if self.rotation == Rotation::Angle {
					None
				} else {
					Some(Rect::new(
						x_pos as f32,
						y_pos as f32,
						size as f32,
						size as f32,
					))
				},
				flip_x: self.obj.axis_horizontal == Axis::Negative
					&& self.rotation == Rotation::EightWay,
				dest_size: Some(Vec2::new(
					size as f32 * SCREEN_SCALE as f32,
					size as f32 * SCREEN_SCALE as f32,
				)),
				..Default::default()
			}),
		)
	}

	fn texture_angle(&mut self, x_pos: u32, y_pos: u32) -> DynamicImage {
		if let Some((old_pos, texture)) = &self.cache {
			if *old_pos == x_pos {
				return texture.clone();
			} else {
				self.cache = None
			}
		}

		let img = DynamicImage::ImageRgba8(rotate_about_center(
			self.sprite
				.crop_imm(x_pos, y_pos, self.sprite.height(), self.sprite.height())
				.as_rgba8()
				.unwrap(),
			(self.obj.target.y - self.obj.pos.y).atan2(self.obj.target.x - self.obj.pos.x) as f32,
			imageproc::geometric_transformations::Interpolation::Nearest,
			Rgba([0, 0, 0, 0]),
		));

		self.cache = Some((x_pos, img.clone()));

		img
	}
}

impl Frames {
	pub fn new_entity() -> Self {
		Self {
			frame_order: vec![0, 1, 0, 2],
			frame_time: 16.,

			anim_time: 0.,
			anim_completed: false,
		}
	}

	pub fn new_attack() -> Self {
		Self {
			frame_order: vec![0, 1, 2],
			frame_time: 4.,

			anim_time: 0.,
			anim_completed: false,
		}
	}

	pub fn new_static() -> Self {
		Self {
			frame_order: vec![0],
			frame_time: 0.,

			anim_time: 0.,
			anim_completed: false,
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
		*self
			.frame_order
			.get((self.anim_time / self.frame_time) as usize)
			.unwrap()
	}
}

impl Default for Frames {
	fn default() -> Self {
		Self {
			frame_order: vec![],
			frame_time: 0.,
			anim_time: 0.,
			anim_completed: false,
		}
	}
}
