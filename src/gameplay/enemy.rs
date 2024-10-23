use macroquad::math::Vec2;
use serde_json::Value;

use super::{player::Player, cores::enemytype::EnemyType, entity::Entity};

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
				if enemy.stats.get_pos().distance(player.stats.get_pos()) < 20. {
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
	/// Creates a new Enemy using a Vec2 for the pos and an EnemyType for the stats
	pub fn new(pos: Vec2, enemytype: EnemyType) -> Self {
		return Self {
			stats: Entity::new(pos, enemytype.size, enemytype.max_health as isize),
			attacks: enemytype.attacks,
			movement: enemytype.movement,
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update(&mut self, player: &mut Player, map: &Vec<Vec2>) {
		self.movement(player, map);

		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}

	/// Moves the enemy based upon their Movement
	fn movement(&mut self, player: &Player, map: &Vec<Vec2>){
		match self.movement {
			// Simple movement AI that tracks the player and moves towards them
			Movement::MoveTowardsPlayer => {
				self.stats.try_move(self.stats.get_pos().move_towards(player.stats.get_pos(), 1.0), map);
			}
		}
	}
}
