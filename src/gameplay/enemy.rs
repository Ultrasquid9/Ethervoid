use macroquad::math::Vec2;

use super::{player::Player, Entity};

enum Movement {
	MoveTowardsPlayer
}

impl Movement {
	fn update(&self, enemy: &Enemy, player: &Player) -> Vec2 {
		match &self {
			Self::MoveTowardsPlayer => {
				let mut new_pos = Vec2::new(0., 0.);

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

enum Attacks {
	ContactDamage
}

impl Attacks {
	fn attack(&self, enemy: &Enemy, player: &Player) -> usize {
		match &self {
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

pub struct Enemy {
	pub stats: Entity,
	movement: Movement,
	attacks: Vec<Attacks>
}

impl Enemy {
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

	pub fn update(&mut self, player: &mut Player) {
		self.stats.pos += self.movement.update(self, player);

		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}
}