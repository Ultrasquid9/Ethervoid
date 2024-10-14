use macroquad::math::Vec2;

use super::{player::Player, Entity};

/// The movement AI used by an enemy
enum Movement {
	MoveTowardsPlayer
}

impl Movement {
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
enum Attacks {
	ContactDamage
}

impl Attacks {
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
	/// Creates a basic test enemy
	pub fn new() -> Self {
		return Enemy {
			stats: Entity {
				health: 10,
				pos: Vec2::new(25., 25.)
			},
			movement: Movement::MoveTowardsPlayer,
			attacks: vec![Attacks::ContactDamage]
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update(&mut self, player: &mut Player) {
		self.stats.pos = self.movement.update(self, player);

		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}

	pub fn damage(&mut self, val: isize) {
		self.stats.health -= val;
	}

	pub fn should_kill(&self) -> bool {
		if self.stats.health <= 0 {
			return true
		} else {
			return false
		}
	}
}