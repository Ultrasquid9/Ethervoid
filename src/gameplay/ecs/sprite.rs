use macroquad::texture::Texture2D;

use crate::{gameplay::draw::{process::{downscale, to_texture}, render::render_texture}, utils::resources::textures::access_image};

use super::obj::Obj;

#[derive(Clone)]
pub struct Sprite {
	pub sprite: Texture2D,
	pub obj: Obj,
}

impl Sprite {
	pub fn new(obj: &Obj) -> Self {
		Self {
			sprite: to_texture(downscale(
				access_image("default:appl"),
				obj.size as u32
			)),
			obj: *obj
		}
	}

	pub fn update(&mut self, new_obj: Obj) {
		self.obj = new_obj;
	}

	pub async fn render(&self) {
		render_texture(&self.sprite, self.obj.pos, None).await;
	}
}
