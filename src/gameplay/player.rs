use macroquad::{input::is_key_down, math::Vec2};
use serde_json::Value;

use crate::input::{get_config, get_keycode};

pub struct Player {
	pub pos: Vec2,
	pub health: u8,

	pub config: Value,
	
	speed: f32
}

impl Player {
	pub fn new() -> Self {
		return Player {
			pos: Vec2::new(0.0, 0.0),
			health: 100,

			config: get_config("./config.json"),
			speed: 0.0
		}
	}

	pub fn update(&mut self) -> &Self {
		if self.health == 0 {
			*self = Self::new();
			return self;
		}

		let mut new_pos = Vec2::new(self.pos.x, self.pos.y);

		if is_key_down(get_keycode(&self.config, "Up")) {
			new_pos.y -= self.speed;
		}
		if is_key_down(get_keycode(&self.config, "Down")) {
			new_pos.y += self.speed;
		}
		if is_key_down(get_keycode(&self.config, "Left")) {
			new_pos.x -= self.speed;
		}
		if is_key_down(get_keycode(&self.config, "Right")) {
			new_pos.x += self.speed;
		}

		if self.speed < 3.0 && new_pos != self.pos {
			self.speed = self.speed + (self.speed / 6.0);
		}

		if new_pos == self.pos {
			self.speed = 1.0;
			return self;
		} else if self.pos.x != new_pos.x && self.pos.y != new_pos.y {
			self.pos = new_pos.midpoint(new_pos.midpoint(self.pos));
		} else {
			self.pos = new_pos;
		}

		return self;
	}
}