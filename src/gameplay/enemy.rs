use macroquad::math::Vec2;

use super::{player::{self, Player}, Entity};

enum Movement {
	TestAI
}

impl Movement {
	fn movement(&self) -> Vec2 {
		return Vec2::new(0., 0.)
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
			movement: Movement::TestAI,
			attacks: vec![Attacks::ContactDamage]
		}
	}

	pub fn update(&mut self, player: &mut Player) {
		for i in &self.attacks {
			player.stats.health -= i.attack(&self, &player);
		}
	}
}