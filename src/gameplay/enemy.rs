use macroquad::math::Vec2;
use serde_json::Value;

use super::{player::Player, builders::enemybuilder::EnemyBuilder, Entity};

/// The movement AI used by an enemy
#[derive(Clone)]
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

	/// Moves the enemy based upon their Movement
	fn update(&self, enemy: &Enemy, player: &Player) -> Vec2 {
		match &self {
			// Simple movement AI that tracks the player and moves towards them
			Self::MoveTowardsPlayer => {
				return enemy.stats.pos.move_towards(player.stats.pos, 1.0)
			}
		}
	}
}

/// The attacks used by an enemy
#[derive(Clone)]
pub enum Attacks {
	ContactDamage
}

impl Attacks {
	/// Provides an Attack enum based on the provided Vector
	pub fn from_vec(input: &Vec<Value>) -> Vec<Attacks> {
		let mut attacks: Vec<Attacks> = Vec::new();

		for i in input{
			match i.as_str().unwrap() {
				"ContactDamage" => attacks.push(Attacks::ContactDamage),

				_ => attacks.push(Attacks::ContactDamage)
			}
		}

		return attacks;
	}

	/// Attacks the player based upon their attacks
	fn attack(&self, enemy: &Enemy, player: &Player) -> isize {
		match &self {
			// Simple attack that damages the player if they are too close
			Self::ContactDamage => {
				if enemy.stats.pos.distance(player.stats.pos) < 20. {
					return 1;
				} else {
					return 0;
				}
			}
		}
	}
}

/// An enemy
pub struct Enemy {
	pub stats: Entity,
	movement: Movement,
	attacks: Vec<Attacks>
}

impl Enemy {
	/// Creates a new Enemy using a Vec2 for the pos and an EnemyBuilder for the stats
	pub fn from_builder(pos: Vec2, builder: EnemyBuilder) -> Self {
		return Self {
			stats: Entity {
				pos: pos,
				health: builder.max_health as isize,
			},
			attacks: builder.attacks,
			movement: builder.movement,
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update(&mut self, player: &mut Player) {
		self.stats.pos = self.movement.update(self, player);

		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}

	/// Damages the enemy based on the provided isize
	pub fn damage(&mut self, val: isize) {
		self.stats.health -= val;
	}

	/// Checks if the enemy is dead
	pub fn should_kill(&self) -> bool {
		if self.stats.health <= 0 {
			return true
		} else {
			return false
		}
	}
}