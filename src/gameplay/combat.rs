use macroquad::math::Vec2;

use super::{enemy::Enemy, player::Player};

pub struct Attack {
	attack_type: AttackType,
	damage: isize,
	pos: Vec2
}

pub enum AttackType {
	Physical(f32),
	Burst(f32),
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

pub struct ProjectileOrHitscan {
	target: Vec2
}

impl Attack {
	pub fn new_physical(pos: Vec2, damage: isize, radius: f32) -> Attack {
		return Attack {
			attack_type: AttackType::Physical(radius),
			pos,
			damage
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, radius: f32) -> Attack {
		return Attack {
			attack_type: AttackType::Burst(radius),
			pos,
			damage
		}
	}

	pub fn new_projectile(pos: Vec2, target: Vec2, damage:isize) -> Attack {
		return Attack {
			attack_type: AttackType::Projectile( ProjectileOrHitscan {
				target
			}),
			pos,
			damage
		}
	}

	pub fn new_hitscan(pos: Vec2, target: Vec2, damage: isize) -> Attack {
		return Attack {
			attack_type: AttackType::Hitscan( ProjectileOrHitscan {
				target
			}),
			pos,
			damage
		}
	}

	pub fn damage(&self, enemies: &mut Vec<Enemy>, _player: &Player) {
		match &self.attack_type {
			AttackType::Physical(attributes) => {
				for i in enemies {
					if i.stats.get_pos().distance(self.pos) <= i.stats.size + attributes {
						i.stats.health -= self.damage;
					}
				}
			},
			AttackType::Burst(attributes) => todo!(),
			AttackType::Projectile(attributes) => todo!(),
			AttackType::Hitscan(attributes) => todo!()
		}
	}
}
