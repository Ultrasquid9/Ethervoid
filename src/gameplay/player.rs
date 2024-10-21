use macroquad::math::Vec2;
use serde_json::Value;

use crate::input::{get_config, is_down, is_pressed};

use super::entity::Entity;

/// Contains info about the player
pub struct Player {
	pub stats: Entity,
	pub config: Value,

	pub swords: [WeaponInfo; 3],
	pub guns: [WeaponInfo; 3],
	pub current_sword: u8,
	pub current_gun: u8, 

	speed: f32,
	axis_horizontal: Axis,
	axis_vertical: Axis
}

/// Contains info about one of the player's weapons
pub struct WeaponInfo {
	weapon: Weapon,
	unlocked: bool,
	pub cooldown: u8
}

#[derive(PartialEq)]
enum Axis {
	Positive,
	Negative,
	None
}

pub enum Weapon {
	// Swords
	Sword,
	Hammer,
	Boomerang,

	// Gunssword_
	Pistol,
	Shotgun,
	RadioCannon
}

impl Player {
	pub fn new() -> Self {
		return Player {
			stats: Entity::new(Vec2::new(0.0, 0.0), 15., 100),
			config: get_config("./config.json"),

			swords: [
				WeaponInfo {weapon: Weapon::Sword, unlocked: true, cooldown: 0},
				WeaponInfo {weapon: Weapon::Hammer, unlocked: true, cooldown: 0},
				WeaponInfo {weapon: Weapon::Boomerang, unlocked: true, cooldown: 0}
			],
			guns: [
				WeaponInfo {weapon: Weapon::Pistol, unlocked: true, cooldown: 0},
				WeaponInfo {weapon: Weapon::Shotgun, unlocked: true, cooldown: 0},
				WeaponInfo {weapon: Weapon::RadioCannon, unlocked: true, cooldown: 0}
			],
			current_sword: 0,
			current_gun: 0,

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

		// Weapon cooldown
		for i in self.swords.iter_mut() {
			if i.cooldown > 0 {
				i.cooldown -= 1;
			}
		}
		for i in self.guns.iter_mut() {
			if i.cooldown > 0 {
				i.cooldown -= 1;
			}
		}

		// Changing weapons
		if is_pressed("Change Sword", &self.config) {
			self.current_sword = swap_weapons(&self.current_sword, &self.swords);
		}
		if is_pressed("Change Gun", &self.config) {
			self.current_gun = swap_weapons(&self.current_gun, &self.guns);
		}

		self.movement(map);

		return self;
	}

	/// Handles player movement
	fn movement(&mut self, map: &Vec<Vec2>) {
		// Checks to see if both Up and Down are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if is_down("Up", &self.config)
		&& is_down("Down", &self.config) {
			if is_pressed("Up", &self.config)
			&& self.axis_vertical != Axis::Negative {
				self.axis_vertical = Axis::Negative;
			} 
			if is_pressed("Down", &self.config)
			&& self.axis_vertical != Axis::Positive {
				self.axis_vertical = Axis::Positive;
			} 
		} else if is_down("Up", &self.config) {
			self.axis_vertical = Axis::Negative;
		} else if is_down("Down", &self.config) {
			self.axis_vertical = Axis::Positive;
		} else {
			self.axis_vertical = Axis::None;
		}

		// Checks to see if both Left and Right are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if is_down("Left", &self.config)
		&& is_down("Right", &self.config) {
			if is_pressed("Left", &self.config)
			&& self.axis_vertical != Axis::Negative {
				self.axis_horizontal = Axis::Negative;
			} 
			if is_pressed("Right", &self.config)
			&& self.axis_vertical != Axis::Positive {
				self.axis_horizontal = Axis::Positive;
			} 
		} else if is_down("Left", &self.config) {
			self.axis_horizontal = Axis::Negative;
		} else if is_down("Right", &self.config) {
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

fn swap_weapons(current_weapon: &u8, weapons: &[WeaponInfo; 3]) -> u8 {
	let mut to_return: u8 = *current_weapon;

	loop {
		if *current_weapon >= weapons.len() as u8 {
			to_return = 0;
		}

		if weapons[to_return as usize].unlocked {
			return to_return;
		} else {
			to_return += 1;
		}
	}
}
