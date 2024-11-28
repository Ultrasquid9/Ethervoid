use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::utils::config::Config;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj, sprite::{Frames, Rotation, Sprite}};

#[derive(SplitFields)]
pub struct Player<'a> {
	pub health: Health,
	pub obj: Obj,
	pub behavior: Behavior<'a>,
	pub sprite: Sprite,

	pub inventory: Inventory,

	pub config: Config
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
	pub unlocked: bool,
	pub cooldown: u8
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
			behavior: Behavior::Player,
			sprite: Sprite::new(
				obj, 
				32,
				"default:entity/player/player_spritesheet_wip",
				Rotation::EightWay,
				Frames::new_entity()
			),

			inventory: Inventory {
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
			},

			config: Config::read("./config.ron")
		}
	}
}
