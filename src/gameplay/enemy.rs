use macroquad::math::Vec2;

use super::{player::Player, Entity};

/// The movement AI used by an enemy
enum Movement {
	MoveTowardsPlayer
}

impl Movement {
	/// Moves the enemy based upon their Movement
	fn update(&self, enemy: &Enemy, player: &Player) -> Vec2 {
		let mut new_pos = Vec2::new(0., 0.);

		match &self {
			// Simple movement AI that tracks the player and moves towards them
			Self::MoveTowardsPlayer => {
				if player.stats.pos.x > enemy.stats.pos.x {
					new_pos.x += 1.;
				} else if player.stats.pos.x < enemy.stats.pos.x {
					new_pos.x -= 1.;
				}

				if player.stats.pos.y > enemy.stats.pos.y {
					new_pos.y += 1.;
				} else if player.stats.pos.y < enemy.stats.pos.y {
					new_pos.y -= 1.;
				}

				return new_pos.normalize();
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
	fn attack(&self, enemy: &Enemy, player: &Player) -> usize {
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
		self.stats.pos += self.movement.update(self, player);

		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}
}