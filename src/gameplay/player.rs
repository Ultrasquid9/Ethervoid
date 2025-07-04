use rustc_hash::FxHashMap;
use stecs::prelude::*;

use super::{
	combat::{Attack, Owner},
	ecs::{
		behavior::player::PlayerController,
		health::Health,
		obj::Obj,
		sprite::{Frames, Rotation, Sprite},
	},
};

use crate::{
	gameplay::draw::ui::PlayerUi,
	utils::{mouse_pos, mouse_pos_local, resources::audio::play_random_sound},
};

use macroquad::prelude::*;

#[derive(SplitFields)]
pub struct Player {
	pub health: Health,
	pub obj: Obj,
	pub controller: PlayerController,
	pub sprite: Sprite,

	pub ui: PlayerUi,
	pub inventory: Inventory,
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
	pub cooldown: f64,
}

pub enum Weapon {
	// Swords
	Sword,
	Hammer,
	Boomerang,

	// Gunssword_
	Pistol,
	Shotgun,
	RadioCannon,
}

impl Player {
	pub fn new() -> Self {
		let pos = DVec2::new(0., 0.);
		let obj = Obj::new(pos, pos, 15.);

		Self {
			health: Health::new(100.),
			obj,
			controller: PlayerController::default(),
			sprite: Sprite::new(
				obj,
				"default:entity/player/player_spritesheet_wip",
				Rotation::EightWay,
				Frames::new_entity(),
				FxHashMap::default(),
			),

			ui: PlayerUi::new(
				"default:ui/hp",
				"default:ui/hp_bar",
				"default:ui/temp",
				"default:ui/temp_bar_first",
				"default:ui/temp_bar_second",
				"default:ui/temp_bar_third",
			),
			inventory: Inventory {
				swords: [
					WeaponInfo {
						weapon: Weapon::Sword,
						unlocked: true,
						cooldown: 0.,
					},
					WeaponInfo {
						weapon: Weapon::Hammer,
						unlocked: true,
						cooldown: 0.,
					},
					WeaponInfo {
						weapon: Weapon::Boomerang,
						unlocked: true,
						cooldown: 0.,
					},
				],
				guns: [
					WeaponInfo {
						weapon: Weapon::Pistol,
						unlocked: true,
						cooldown: 0.,
					},
					WeaponInfo {
						weapon: Weapon::Shotgun,
						unlocked: true,
						cooldown: 0.,
					},
					WeaponInfo {
						weapon: Weapon::RadioCannon,
						unlocked: true,
						cooldown: 0.,
					},
				],
				current_sword: 0,
				current_gun: 0,
			},
		}
	}
}

impl Inventory {
	// Gets a sword attack based upon the currently selected sword
	pub fn attack_sword(&mut self, pos: DVec2) -> Attack {
		match self.swords[self.current_sword].weapon {
			Weapon::Sword => {
				play_random_sound(&[
					"default:sfx/sword_1",
					"default:sfx/sword_2",
					"default:sfx/sword_3",
				]);

				self.swords[self.current_sword].cooldown = 16.;
				Attack::new_physical(
					Obj::new(pos, pos + mouse_pos_local(), 36.),
					10.,
					Owner::Player,
					"default:attacks/slash",
				)
			}
			Weapon::Hammer => {
				self.swords[self.current_sword].cooldown = 32.;
				Attack::new_burst(
					Obj::new(pos, pos, 36.),
					10.,
					Owner::Player,
					"default:attacks/burst",
				)
			}
			Weapon::Boomerang => {
				self.swords[self.current_sword].cooldown = 48.;
				Attack::new_projectile(
					Obj::new(pos, mouse_pos() * 999., 10.),
					10.,
					Owner::Player,
					"default:attacks/projectile-player",
				)
			}

			_ => unreachable!("Gun in Sword position"),
		}
	}

	// Gets a gun attack based upon the currently selected gun
	pub fn attack_gun(&mut self, pos: DVec2) -> Attack {
		match self.guns[self.current_gun].weapon {
			Weapon::Pistol => {
				self.guns[self.current_gun].cooldown = 16.;
				Attack::new_projectile(
					Obj::new(pos, mouse_pos() * 999., 6.),
					10.,
					Owner::Player,
					"default:attacks/projectile-player",
				)
			}
			Weapon::Shotgun => Attack::new_burst(
				Obj::new(pos, pos, 16.),
				10.,
				Owner::Player,
				"default:attacks/burst",
			),
			Weapon::RadioCannon => {
				self.guns[self.current_gun].cooldown = 48.;
				Attack::new_hitscan(
					Obj::new(pos, mouse_pos() * 999., 6.),
					6.,
					Owner::Player,
					"default:attacks/hitscan-player",
				)
			}

			_ => unreachable!("Sword in Gun position"),
		}
	}
}

pub fn swap_weapons(current_weapon: usize, weapons: &[WeaponInfo]) -> usize {
	let mut to_return: usize = current_weapon;

	loop {
		to_return += 1;
		if current_weapon >= weapons.len() - 1 {
			to_return = 0;
		}

		if weapons[to_return].unlocked {
			return to_return;
		}
	}
}
