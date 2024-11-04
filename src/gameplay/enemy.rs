use macroquad::{math::Vec2, texture::Texture2D};
use serde::Deserialize;

use super::{combat::Attack, cores::{attackscript::AttackScript, enemytype::EnemyType}, draw::texturedentity::{Texture, TexturedEntity}, entity::{Entity, MovableObj}, player::Player};

/// The movement AI used by an enemy
#[derive(Clone, Deserialize)]
pub enum Movement {
	MoveTowardsPlayer
}

impl Movement {
	/// Provides a Movement enum based on the provided String
	pub fn from_str(input: &str) -> Movement {
		match input {
			"MoveTowardsPlayer" => Movement::MoveTowardsPlayer,

			_ => Movement::MoveTowardsPlayer
		}
	}
}

/// An enemy
/// The lifetime annotation allows the compiler to know that the Enemy lives as long as the AttackScript does
pub struct Enemy<'a> {
	pub stats: Entity,
	movement: Movement,

	attacks: Vec<AttackScript<'a>>,
	attack_index: usize,
	attack_cooldown: usize
}

impl Enemy<'_> {
	/// Creates a new Enemy using a Vec2 for the pos and an EnemyType for the stats
	pub fn new(pos: Vec2, enemytype: EnemyType, texture: Texture2D) -> Self {
		return Self {
			stats: Entity::new(pos, enemytype.size, enemytype.max_health as isize, Texture::new(texture)),
			movement: enemytype.movement,
			attacks: enemytype.attacks
				.iter()
				.map(|attack| attack.clone().build())
				.collect(),
			attack_index: 0,
			attack_cooldown: 32
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update<'a>(&'a mut self, attacks: &mut Vec<Attack>, player: &mut Player, map: &Vec<Vec2>) {
		if self.stats.i_frames != 0 {
			self.stats.i_frames -= 1
		}

		for attack in &mut *attacks {
			if attack.is_parried
			&& self.stats.is_touching(attack) {
				self.stats.stunned = 32;
				self.attack_cooldown = 64;
			}
		}

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
			self.movement(player, map);

			self.attacks[self.attack_index].set_target(player.stats.get_pos());
			self.attack_cooldown -= 1;
		}

		self.update_texture();
	}

	/// Moves the enemy based upon their Movement
	fn movement(&mut self, player: &Player, map: &Vec<Vec2>){
		if self.stats.stunned > 0 {
			self.stats.stunned -= 1;
			return;
		}

		match self.movement {
			// Simple movement AI that tracks the player and moves towards them
			Movement::MoveTowardsPlayer => {
				let new_pos = self.stats.get_pos().move_towards(player.stats.get_pos(), 1.0);

				self.stats.update_axis(&new_pos);
				self.stats.try_move(new_pos, map);
			}
		}
	}
}

impl TexturedEntity for Enemy<'_> {
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
