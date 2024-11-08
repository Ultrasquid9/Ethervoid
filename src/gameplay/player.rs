use macroquad::{input::mouse_position_local, math::Vec2};

use crate::config::Config;

use super::{combat::{Attack, Owner}, draw::{access_texture, texturedobj::{AttackTextureType, EntityTexture, TexturedObj}}, entity::{Entity, MovableObj}, get_delta_time, get_mouse_pos};

/// Contains info about the player
pub struct Player {
	pub stats: Entity,
	pub config: Config,

	pub swords: [WeaponInfo; 3],
	pub guns: [WeaponInfo; 3],
	pub current_sword: usize,
	pub current_gun: usize, 

	speed: f32,
	dash_cooldown: f32,
	pub is_dashing: bool,

	axis_horizontal: Axis,
	axis_vertical: Axis
}

/// Contains info about one of the player's weapons
pub struct WeaponInfo {
	weapon: Weapon,
	unlocked: bool,
	pub cooldown: u8
}

#[derive(PartialEq, Clone)]
pub enum Axis {
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
			stats: Entity::new(
				Vec2::new(0.0, 0.0), 
				15., 
				100, 
				EntityTexture::new(access_texture("default:entity/player/player_spritesheet_wip"))
			),
			config: Config::read("./config.ron"),

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
			dash_cooldown: 0.,
			is_dashing: false,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None
		}
	}

	/// Updates the player
	pub fn update(&mut self, map: &Vec<Vec2>) -> &Self {
		// Handling i-frames
		if self.stats.i_frames != 0 {
			self.stats.i_frames -= 1
		}

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
		if self.config.keymap.change_sword.is_pressed() {
			self.current_sword = swap_weapons(&self.current_sword, &self.swords);
		}
		if self.config.keymap.change_gun.is_pressed() {
			self.current_gun = swap_weapons(&self.current_gun, &self.guns);
		}

		self.update_texture();
		self.movement(map);

		return self;
	}

	// Gets a sword attack based upon the currently selected sword
	pub fn attack_sword(&mut self) -> Attack {
		match self.swords[self.current_sword].weapon {
			Weapon::Sword => {
				self.swords[self.current_sword].cooldown = 16;
				Attack::new_physical(
					self.stats.get_pos(), 
					mouse_position_local(), 
					10, 
					36., 
					Owner::Player, 
					AttackTextureType::Slash
				)
			},
			Weapon::Hammer => {
				self.swords[self.current_sword].cooldown = 32;
				Attack::new_burst(
					self.stats.get_pos(), 
					10, 
					36., 
					Owner::Player,
					AttackTextureType::Burst
				)
			},
			Weapon::Boomerang => {
				self.swords[self.current_sword].cooldown = 48;
				Attack::new_projectile(
					self.stats.get_pos(), 
					get_mouse_pos() * 999., 
					10, 
					Owner::Player, 
					AttackTextureType::ProjectilePlayer
				)
			},
			
			_ => panic!("Bad weapon")
		}
	}

	// Gets a gun attack based upon the currently selected gun
	pub fn attack_gun(&mut self) -> Attack {
		match self.guns[self.current_gun].weapon {
			Weapon::Pistol => {
				self.guns[self.current_gun].cooldown = 16;
				Attack::new_projectile(
					self.stats.get_pos(), 
					get_mouse_pos() * 999., 
					10, 
					Owner::Player,
					AttackTextureType::ProjectilePlayer
				)
			},
			Weapon::Shotgun => {
				self.guns[self.current_gun].cooldown = 32;
				Attack::new_burst(
					self.stats.get_pos(), 
					10, 
					36., 
					Owner::Player,
					AttackTextureType::Burst
				)
			},
			Weapon::RadioCannon => {
				self.guns[self.current_gun].cooldown = 48;
				Attack::new_hitscan(self.stats.get_pos(), get_mouse_pos() * 999., 6, Owner::Player)
			},
			
			_ => panic!("Bad weapon")
		}
	}

	/// Handles player movement
	fn movement(&mut self, map: &Vec<Vec2>) {
		// Checks to see if both Up and Down are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if self.config.keymap.up.is_down()
		&& self.config.keymap.down.is_down() {
			if self.config.keymap.up.is_pressed()
			&& self.axis_vertical != Axis::Negative {
				self.axis_vertical = Axis::Negative;
			} 
			if self.config.keymap.down.is_pressed()
			&& self.axis_vertical != Axis::Positive {
				self.axis_vertical = Axis::Positive;
			} 
		} else if self.config.keymap.up.is_down() {
			self.axis_vertical = Axis::Negative;
		} else if self.config.keymap.down.is_down() {
			self.axis_vertical = Axis::Positive;
		} else {
			self.axis_vertical = Axis::None;
		}

		// Checks to see if both Left and Right are being held at the same time.
		// If they are, sets the direction to move based upon the most recently pressed key. 
		// Otherwise, sets the direction to move based upon the currently pressed key.
		if self.config.keymap.left.is_down()
		&& self.config.keymap.right.is_down() {
			if self.config.keymap.left.is_pressed()
			&& self.axis_vertical != Axis::Negative {
				self.axis_horizontal = Axis::Negative;
			} 
			if self.config.keymap.right.is_pressed()
			&& self.axis_vertical != Axis::Positive {
				self.axis_horizontal = Axis::Positive;
			} 
		} else if self.config.keymap.left.is_down() {
			self.axis_horizontal = Axis::Negative;
		} else if self.config.keymap.right.is_down() {
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

		// Dashing
		if self.config.keymap.dash.is_down() && self.dash_cooldown <= 0.{
			self.speed += 10.;
			self.dash_cooldown += 70.;
		} else if self.dash_cooldown > 0. {
			if self.dash_cooldown > 55. {
				self.is_dashing = true;
				self.speed = 10.;
			} else {
				self.is_dashing = false;
			}
			self.dash_cooldown -= get_delta_time();
		}

		// Makes the player build up speed over time, rather than instantly starting at max speed
		if self.speed < 3. && new_pos != Vec2::new(0., 0.) {
			self.speed += self.speed / 6.;
		}

		// Makes the player slow down if their speed is high
		if self.speed > 3.5 {
			self.speed = self.speed / 1.5;
		}

		// Checks to see if the player has moved. 
		// If they have not, resets the speed. 
		// If they have, attempts to move to the new position. 
		if new_pos == Vec2::new(0., 0.) {
			self.speed = 1.0;
		} else {
			let current_pos = self.stats.get_pos();
			self.stats.try_move((new_pos.normalize() * self.speed * get_delta_time()) + current_pos, map);
		}
	}
}

impl TexturedObj for Player {
	fn update_texture(&mut self) {
		self.stats.texture.update(
			self.stats.get_pos(),
			self.axis_horizontal, 
			self.axis_vertical, 
			if self.speed > 1.0 {
				true
			} else {
				false
			}
		);
	}
}

impl Copy for Axis {} // Apparently I dont even need to do anything

fn swap_weapons(current_weapon: &usize, weapons: &[WeaponInfo; 3]) -> usize {
	let mut to_return: usize = *current_weapon;

	loop {
		to_return += 1;
		if *current_weapon >= weapons.len() - 1 {
			to_return = 0;
		}

		if weapons[to_return].unlocked {
			return to_return;
		}
	}
}
