use rustc_hash::FxHashMap;

use crate::{
	gameplay::draw::process::{scale, to_texture},
	utils::{
		angle_between,
		error::{EtherVoidError, EvoidResult},
		resources::{config::access_config, textures::access_image},
		smart_time,
	},
};

use macroquad::prelude::*;

use imageproc::{
	geometric_transformations::rotate_about_center,
	image::{DynamicImage, Rgba},
};

use serde::{Deserialize, Serialize};

use super::obj::{Axis, Obj};

#[derive(Clone)]
pub struct Sprite {
	img: DynamicImage,
	obj: Obj,

	cache: Option<(u32, Texture2D)>,

	rotation: Rotation,
	frames: Frames,

	shaking: f64,

	current_anim: Option<String>,
	anims: FxHashMap<String, Frames>,
}

#[derive(Clone, Copy, PartialEq)]
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
		anims: FxHashMap<String, Frames>,
	) -> Self {
		Self {
			img: if rotation == Rotation::Angle {
				scale(access_image(key), obj.size as u32)
			} else {
				access_image(key).clone()
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
			self.shaking -= smart_time();
		}

		if let Some(ref anim) = self.current_anim {
			if let Some(anim) = self.anims.get_mut(anim) {
				anim.update();
			}

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

	pub fn get_current_anim(&mut self) -> Option<&str> {
		self.current_anim.as_deref()
	}

	pub fn set_new_anim(&mut self, key: String) -> EvoidResult<()> {
		if !self.anims.contains_key(&key) {
			return Err(EtherVoidError::AnimNotFound(key).into());
		}

		self.current_anim = Some(key);

		self.frames.reset();
		for anim in self.anims.values_mut() {
			anim.reset();
		}

		Ok(())
	}

	pub fn set_img(&mut self, img: DynamicImage) {
		self.img = img;
		self.cache = None;
	}

	pub fn img(&self) -> &DynamicImage {
		&self.img
	}

	pub fn set_rotation(&mut self, rotation: Rotation) {
		self.rotation = rotation;
		self.cache = None;
	}

	pub fn rotation(&self) -> Rotation {
		self.rotation
	}

	pub fn set_default_anim(&mut self) {
		self.current_anim = None;
	}

	pub fn as_render_params(&mut self) -> (Texture2D, DVec2, Option<DrawTextureParams>) {
		let screen_scale = access_config().screen_scale;

		let size = if self.rotation == Rotation::EightWay {
			self.img.height() / 5
		} else {
			self.img.height()
		};

		let mut x_pos = if let Some(ref anim) = self.current_anim {
			self.anims
				.get(anim)
				.map(Frames::get_frame)
				.unwrap_or_default()
		} else {
			self.frames.get_frame()
		} * size;

		let mut y_pos = if self.rotation == Rotation::EightWay {
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
		} else {
			0
		};

		if self.shaking > 0. {
			x_pos += (self.shaking.sin() * 3.) as u32;
		}

		x_pos = x_pos.clamp(0, self.img.width() - 1);
		y_pos = y_pos.clamp(0, self.img.height() - 1);

		(
			if self.rotation == Rotation::Angle {
				self.texture_angle(x_pos, y_pos)
			} else {
				self.texture_non_angle()
			},
			dvec2(
				self.obj.pos.x
					+ match self.rotation {
						Rotation::Angle => 0.,
						Rotation::Static => self.img.width() as f64 / 2.,
						Rotation::EightWay => self.img.width() as f64,
					},
				self.obj.pos.y
					+ match self.rotation {
						Rotation::Angle => 0.,
						Rotation::Static => self.img.width() as f64 / 2.,
						Rotation::EightWay => self.img.height() as f64,
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
				dest_size: Some(vec2(
					size as f32 * screen_scale as f32,
					size as f32 * screen_scale as f32,
				)),
				..Default::default()
			}),
		)
	}

	fn texture_angle(&mut self, x_pos: u32, y_pos: u32) -> Texture2D {
		if let Some((old_pos, texture)) = &self.cache {
			if *old_pos == x_pos {
				return texture.clone();
			}
			self.cache = None;
		}

		let img = to_texture(&DynamicImage::ImageRgba8(rotate_about_center(
			self.img
				.crop_imm(x_pos, y_pos, self.img.height(), self.img.height())
				.as_rgba8()
				.expect("All textures should be RGBA8!"),
			angle_between(&self.obj.pos, &self.obj.target) as f32,
			imageproc::geometric_transformations::Interpolation::Nearest,
			Rgba([0, 0, 0, 0]),
		)));

		self.cache = Some((x_pos, img.clone()));

		img
	}

	fn texture_non_angle(&mut self) -> Texture2D {
		if let Some((_, texture)) = &self.cache {
			return texture.clone();
		}

		let texture = to_texture(&self.img);
		self.cache = Some((0, texture.clone()));
		texture
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
		self.anim_time += smart_time();

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
			.unwrap_or(&0)
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
