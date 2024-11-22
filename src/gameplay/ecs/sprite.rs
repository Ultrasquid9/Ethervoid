use macroquad::{color::WHITE, texture::{draw_texture, Texture2D}};

use crate::{gameplay::draw::to_texture, utils::resources::textures::access_image};

use super::obj::Obj;

pub struct Sprite {
	pub sprite: Texture2D,
	pub obj: Obj,
}

impl Sprite {
	pub fn new(obj: &Obj) -> Self {
		Self {
			sprite: to_texture(access_image("default:appl")),
			obj: obj.clone()
		}
	}

	pub fn update(&mut self, new_obj: Obj) {
		self.obj = new_obj;
	}

	pub fn render(&self) {
		draw_texture(&self.sprite, self.obj.pos.x, self.obj.pos.y, WHITE);
	}
}
