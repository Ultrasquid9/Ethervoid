use image::{DynamicImage, Rgba};
use imageproc::geometric_transformations::rotate_about_center;

use crate::{gameplay::draw::{process::{downscale, to_texture}, render::render_texture}, utils::resources::textures::access_image};

use super::obj::Obj;

#[derive(Clone)]
pub struct Sprite {
	pub sprite: DynamicImage,
	pub obj: Obj,
	pub spritetype: SpriteType,

	anim_time: f32,
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum SpriteType {
	Rotated,
	EightWay,
	Static
}

impl Sprite {
	pub fn new(obj: Obj, key: &str, spritetype: SpriteType) -> Self {
		Self {
			sprite: downscale(
				access_image(key),
				obj.size as u32
			),
			obj,
			spritetype,

			anim_time: 0.
		}
	}

	pub fn update(&mut self, new_obj: Obj) {
		self.obj = new_obj;
	}

	pub async fn render(&self) {
		render_texture(
			&to_texture(if self.spritetype == SpriteType::Rotated {
				DynamicImage::ImageRgba8(rotate_about_center(
					self.sprite.as_rgba8().unwrap(), 
					self.obj.pos.angle_between(self.obj.target), 
					imageproc::geometric_transformations::Interpolation::Nearest, 
					Rgba([0, 0, 0, 0])
				))
			} else {
				self.sprite.clone()
			}),
			self.obj.pos, 
			None
		).await;
	}
}
