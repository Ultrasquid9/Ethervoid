use macroquad::{input::is_key_down, math::Vec2};
use serde_json::Value;

use crate::input::{get_config, get_keycode};

use super::movement::Entity;

pub struct Player {
	pub stats: Entity,
	pub config: Value,
	speed: f32
}

impl Player {
	pub fn new() -> Self {
		return Player {
			stats: Entity::new(Vec2::new(0.0, 0.0), 15., 100),
			config: get_config("./config.json"),
			speed: 0.0
		}
	}

	pub fn update(&mut self, map: &Vec<Vec2>) -> &Self {
		if self.stats.health <= 0 {
			*self = Self::new();
			return self;
		}

		let mut new_pos = Vec2::new(0., 0.);

		if is_key_down(get_keycode(&self.config, "Up")) {
			new_pos.y -= 1.;
		}
		if is_key_down(get_keycode(&self.config, "Down")) {
			new_pos.y += 1.;
		}
		if is_key_down(get_keycode(&self.config, "Left")) {
			new_pos.x -= 1.;
		}
		if is_key_down(get_keycode(&self.config, "Right")) {
			new_pos.x += 1.;
		}

		if self.speed < 3.0 && new_pos != Vec2::new(0., 0.) {
			self.speed = self.speed + (self.speed / 6.0);
		}

		if new_pos == Vec2::new(0., 0.) {
			self.speed = 1.0;
		} else {
			self.stats.try_move((new_pos.normalize() * self.speed) + self.stats.get_pos(), map);
		}

		return self;
	}
}