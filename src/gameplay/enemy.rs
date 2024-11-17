use std::cmp::Ordering;

use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::utils::resources::access_texture;

use super::{cores::{behavior::Behavior, enemytype::EnemyType, map::Map}, draw::texturedobj::{EntityTexture, TexturedObj}, ecs::Attacks, entity::{Entity, MovableObj}, player::Player};

/// An enemy
pub struct Enemy {
	pub stats: Entity,
	movement: Behavior<'static>,

	attacks: Vec<Behavior<'static>>,
	attack_index: usize,
	attack_cooldown: usize
}

impl Enemy {
	/// Creates a new Enemy using a Vec2 for the pos and an EnemyType for the stats
	pub fn new(pos: Vec2, enemytype: EnemyType) -> Self {
		return Self {
			stats: Entity::new(
				pos, 
				enemytype.size, 
				enemytype.max_health as isize, 
				EntityTexture::new(access_texture(&enemytype.sprite))
			),
			movement: enemytype.movement.build(),
			attacks: enemytype.attacks
				.iter()
				.map(|attack| attack.clone().build())
				.collect(),
			attack_index: 0,
			attack_cooldown: 32
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update(&mut self, attacks: &mut Attacks, player: &mut Player, map: &Map) {
		if self.stats.i_frames != 0 {
			self.stats.i_frames -= 1
		}

		for attack in query!(attacks, (&io)) {
			if attack.is_parried
			&& self.stats.is_touching(attack) {
				self.stats.stunned = 32;
				self.attack_cooldown = 64;
			}
		}

		self.movement(attacks, player, map);

		self.update_texture();
	}

	/// Moves the enemy
	/// 
	/// Attacks if possible, otherwise just executes a movement script 
	fn movement(self: &mut Self, attacks: &mut Attacks, player: &mut Player, map: &Map) {
		if self.attack_cooldown == 0 
		&& self.stats.stunned == 0 {
			if self.attacks[self.attack_index].read_script(&mut self.stats, player, map, attacks) {
				self.attack_cooldown = 64;

				if self.attack_index == self.attacks.len() - 1 {
					self.attack_index = 0;
				} else {
					self.attack_index += 1;
				}
			}
		} else {
			if self.stats.stunned > 0 {
				self.stats.stunned -= 1;
			} else {
				self.movement.read_script(&mut self.stats, player, map, attacks);
			}			

			self.attacks[self.attack_index].current_target = player.stats.get_pos();
			self.attack_cooldown -= 1;
		}
	}
}

impl Eq for Enemy {}

impl PartialEq for Enemy {
	fn eq(&self, other: &Self) -> bool {
		if self.stats == other.stats 
		&& self.attack_cooldown == other.attack_cooldown
		&& self.attack_index == other.attack_index {
			return true
		}
		return false
	}
}

impl PartialOrd for Enemy {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.stats.get_pos().y > other.stats.get_pos().y {
			return Some(Ordering::Greater)
		} else if self.stats.get_pos().y < other.stats.get_pos().y {
			return Some(Ordering::Less)
		} else {
			return Some(Ordering::Equal)
		}
	}
}

impl Ord for Enemy {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.stats.get_pos().y > other.stats.get_pos().y {
			return Ordering::Greater
		} else if self.stats.get_pos().y < other.stats.get_pos().y {
			return Ordering::Less
		} else {
			return Ordering::Equal
		}
	}
}

impl TexturedObj for Enemy {
	fn update_texture(&mut self) {
		self.stats.texture.update(
			self.stats.get_pos(), 
			self.stats.dir_horizontal, 
			self.stats.dir_vertical, 
			if self.stats.stunned == 0 {
				true
			} else {
				false
			}
		);
	}
}
