use macroquad::{input::{is_key_down, is_key_pressed}, math::Vec2};
use serde_json::Value;

use crate::input::{get_config, get_keycode};

use super::entity::Entity;

pub struct Player {
	pub stats: Entity,
	pub config: Value,

	pub sword_cooldown: u8,
	pub gun_cooldown: u8,

	speed: f32,
	axis_horizontal: Axis,
	axis_vertical: Axis
}

#[derive(PartialEq)]
enum Axis {
	Positive,
	Negative,
	None
}

impl Player {
	pub fn new() -> Self {
		return Player {
			stats: Entity::new(Vec2::new(0.0, 0.0), 15., 100),
			config: get_config("./config.json"),

			sword_cooldown: 0,
			gun_cooldown: 0,

			speed: 1.,
			axis_horizontal: Axis::None,
			axis_vertical: Axis::None
		}
	}

	/// Updates the player
	pub fn update(&mut self, map: &Vec<Vec2>) -> &Self {
		// Death code. WIP. 
		if self.stats.should_kill() {
			*self = Self::new();
			return self;
		}

		if self.sword_cooldown != 0 {
			self.sword_cooldown -= 1;
		}
		if self.gun_cooldown != 0 {
			self.gun_cooldown -= 1;
		}

		self.movement(map);

		return self;
	}

	/// Handles player movement
	fn movement(&mut self, map: &Vec<Vec2>) {
		// Checks to see if both Up and Down are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if is_key_down(get_keycode(&self.config, "Up")) 
		&& is_key_down(get_keycode(&self.config, "Down")) {
			if is_key_pressed(get_keycode(&self.config, "Up")) 
			&& self.axis_vertical != Axis::Negative {
				self.axis_vertical = Axis::Negative;
			} 
			if is_key_pressed(get_keycode(&self.config, "Down")) 
			&& self.axis_vertical != Axis::Positive {
				self.axis_vertical = Axis::Positive;
			} 
		} else if is_key_down(get_keycode(&self.config, "Up")) {
			self.axis_vertical = Axis::Negative;
		} else if is_key_down(get_keycode(&self.config, "Down")) {
			self.axis_vertical = Axis::Positive;
		} else {
			self.axis_vertical = Axis::None;
		}

		// Checks to see if both Left and Right are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if is_key_down(get_keycode(&self.config, "Left")) 
		&& is_key_down(get_keycode(&self.config, "Right")) {
			if is_key_pressed(get_keycode(&self.config, "Left")) 
			&& self.axis_vertical != Axis::Negative {
				self.axis_horizontal = Axis::Negative;
			} 
			if is_key_pressed(get_keycode(&self.config, "Right")) 
			&& self.axis_vertical != Axis::Positive {
				self.axis_horizontal = Axis::Positive;
			} 
		} else if is_key_down(get_keycode(&self.config, "Left")) {
			self.axis_horizontal = Axis::Negative;
		} else if is_key_down(get_keycode(&self.config, "Right")) {
			self.axis_horizontal = Axis::Positive;
		} else {
			self.axis_horizontal = Axis::None;
		}

		let mut new_pos = Vec2::new(0., 0.); // The pos to be moved to 

		match self.axis_vertical {
			Axis::Positive => new_pos.y += 1.,
			Axis::Negative => new_pos.y -= 1.,
			Axis::None => ()
		}
		match self.axis_horizontal {
			Axis::Positive => new_pos.x += 1.,
			Axis::Negative => new_pos.x -= 1.,
			Axis::None => ()
		}

		// Makes the player build up speed over time, rather than instantly starting at max speed
		if self.speed < 3.0 && new_pos != Vec2::new(0., 0.) {
			self.speed = self.speed + (self.speed / 6.0);
		}

		// Checks to see if the player has moved. 
		// If they have not, resets the speed. 
		// If they have, attempts to move to the new position. 
		if new_pos == Vec2::new(0., 0.) {
			self.speed = 1.0;
		} else {
			self.stats.try_move((new_pos.normalize() * self.speed) + self.stats.get_pos(), map);
		}
	}
}