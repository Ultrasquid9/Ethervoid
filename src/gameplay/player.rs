use macroquad::{input::mouse_position_local, math::Vec2};
use stecs::prelude::*;

use crate::utils::{get_mouse_pos, resources::audio::play_random_sound};

use super::{combat::{Attack, Owner}, ecs::{behavior::{Behavior, PlayerBehavior}, health::Health, obj::Obj, sprite::{Frames, Rotation, Sprite}}};

#[derive(SplitFields)]
pub struct Player<'a> {
	pub health: Health,
	pub obj: Obj,
	pub behavior: Behavior<'a>,
	pub sprite: Sprite,

	pub inventory: Inventory
}

pub struct Inventory {
	pub swords: [WeaponInfo; 3],
	pub guns: [WeaponInfo; 3],
	pub current_sword: usize,
	pub current_gun: usize, 
}

/// Contains info about one of the player's weaponsa
pub struct WeaponInfo {
	pub weapon: Weapon,
	pub _unlocked: bool,
	pub cooldown: f32
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

impl Player<'_> {
	pub fn new() -> Self {
		let pos = Vec2::new(0., 0.);
		let obj = Obj::new(pos, pos, 15.);

		Self {
			health: Health::new(100.),
			obj,
			behavior: Behavior::Player(PlayerBehavior {
				speed: 1.,

				dash_cooldown: 0.,
				is_dashing: false
			}),
			sprite: Sprite::new(
				obj, 
				32,
				"default:entity/player/player_spritesheet_wip",
				Rotation::EightWay,
				Frames::new_entity()
			),

			inventory: Inventory {
				swords: [
					WeaponInfo {weapon: Weapon::Sword, _unlocked: true, cooldown: 0.},
					WeaponInfo {weapon: Weapon::Hammer, _unlocked: true, cooldown: 0.},
					WeaponInfo {weapon: Weapon::Boomerang, _unlocked: true, cooldown: 0.}
				],
				guns: [
					WeaponInfo {weapon: Weapon::Pistol, _unlocked: true, cooldown: 0.},
					WeaponInfo {weapon: Weapon::Shotgun, _unlocked: true, cooldown: 0.},
					WeaponInfo {weapon: Weapon::RadioCannon, _unlocked: true, cooldown: 0.}
				],
				current_sword: 0,
				current_gun: 0,
			}
		}
	}
}

impl Inventory {
	// Gets a sword attack based upon the currently selected sword
	pub fn attack_sword(&mut self, pos: Vec2) -> Attack {
		match self.swords[self.current_sword].weapon {
			Weapon::Sword => {
				play_random_sound(&[
					"default:sfx/sword_1",
					"default:sfx/sword_2",
					"default:sfx/sword_3"
				]);

				self.swords[self.current_sword].cooldown = 16.;
				return Attack::new_physical(
					Obj::new(
						pos, 
						pos + mouse_position_local(), 
						36., 
					),
					10., 
					Owner::Player, 
					"default:attacks/slash"
				)
			},
			Weapon::Hammer => {
				self.swords[self.current_sword].cooldown = 32.;
				return Attack::new_burst(
					Obj::new(
						pos, 
						pos, 
						36., 
					),
					10., 
					Owner::Player, 
					"default:attacks/burst"
				)
			},
			Weapon::Boomerang => {
				self.swords[self.current_sword].cooldown = 48.;
				return Attack::new_projectile(
					Obj::new(
						pos, 
						get_mouse_pos() * 999., 
						10., 
					),
					10., 
					Owner::Player, 
					"default:attacks/projectile-player"
				)
			},
			
			_ => panic!("Bad weapon")
		}
	}

	// Gets a gun attack based upon the currently selected gun
	pub fn attack_gun(&mut self, pos: Vec2) -> Attack {
		match self.guns[self.current_gun].weapon {
			Weapon::Pistol => {
				self.guns[self.current_gun].cooldown = 16.;
				return Attack::new_projectile(
					Obj::new(
						pos, 
						get_mouse_pos() * 999., 
						6., 
					),
					10., 
					Owner::Player, 
					"default:attacks/projectile-player"
				)
			},
			Weapon::Shotgun => {
				return Attack::new_burst(
					Obj::new(
						pos, 
						pos, 
						16., 
					),
					10., 
					Owner::Player, 
					"default:attacks/burst"
				)
			},
			Weapon::RadioCannon => {
				self.guns[self.current_gun].cooldown = 48.;
				return Attack::new_hitscan(
					Obj::new(
						pos, 
						get_mouse_pos() * 999., 
						6., 
					),
					6., 
					Owner::Player
				)
			},
			
			_ => panic!("Bad weapon")
		}
	}
}
