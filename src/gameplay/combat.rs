use macroquad::math::Vec2;

use super::{enemy::Enemy, player::Player};

pub struct Attack {
	attack_type: AttackType,
	damage: isize,
	size: f32,
	pos: Vec2
}

pub enum AttackType {
	Physical,
	Burst,
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

pub struct ProjectileOrHitscan {
	target: Vec2
}

impl Attack {
	pub fn new_physical(pos: Vec2, damage: isize, size: f32) -> Attack {
		return Attack {
			attack_type: AttackType::Physical,
			damage,
			size,
			pos
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, size: f32) -> Attack {
		return Attack {
			attack_type: AttackType::Burst,
			damage,
			size,
			pos
		}
	}

	pub fn new_projectile(pos: Vec2, target: Vec2, damage:isize) -> Attack {
		return Attack {
			attack_type: AttackType::Projectile( ProjectileOrHitscan {
				target
			}),
			damage,
			size: 10.,
			pos,
		}
	}

	pub fn new_hitscan(pos: Vec2, target: Vec2, damage: isize) -> Attack {
		return Attack {
			attack_type: AttackType::Hitscan( ProjectileOrHitscan {
				target
			}),
			damage,
			size: 10.,
			pos
		}
	}

	pub fn damage(&self, enemies: &mut Vec<Enemy>, _player: &Player) {
		match &self.attack_type {
			AttackType::Physical => {
				for i in enemies {
					if i.stats.get_pos().distance(self.pos) <= i.stats.size + self.size {
						i.stats.health -= self.damage;
					}
				}
			},
			AttackType::Burst => {
				for i in enemies {
					if i.stats.get_pos().distance(self.pos) <= i.stats.size + (self.size * 2.) {
						i.stats.health -= self.damage * (i.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize;
					}
				}
			},
			AttackType::Projectile(attributes) => todo!(),
			AttackType::Hitscan(attributes) => todo!()
		}
	}
}
