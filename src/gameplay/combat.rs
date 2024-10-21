use macroquad::math::Vec2;

use super::{enemy::Enemy, player::Player};

pub struct Attack {
	pub size: f32,
	pub pos: Vec2,

	attack_type: AttackType,
	damage: isize,
	lifetime: u8,
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
			pos,
			lifetime: 8,
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, size: f32) -> Attack {
		return Attack {
			attack_type: AttackType::Burst,
			damage,
			size,
			pos,
			lifetime: 12
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
			lifetime: 1,
		}
	}

	pub fn new_hitscan(pos: Vec2, target: Vec2, damage: isize) -> Attack {
		return Attack {
			attack_type: AttackType::Hitscan( ProjectileOrHitscan {
				target
			}),
			damage,
			size: 10.,
			pos,
			lifetime: 8
		}
	}

	pub fn update(&mut self, enemies: &mut Vec<Enemy>, _player: &Player) {
		match &self.attack_type {
			AttackType::Physical => {
				for i in enemies {
					if i.stats.get_pos().distance(self.pos) <= i.stats.size + self.size {
						i.stats.health -= self.damage;
					}
				}
				self.lifetime -= 1;
			},
			AttackType::Burst => {
				for i in enemies {
					if i.stats.get_pos().distance(self.pos) <= i.stats.size + (self.size * 2.) {
						i.stats.health -= self.damage * (i.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize;
					}
				}
				self.lifetime -= 1;
			},
			AttackType::Projectile(attributes) => {
				self.lifetime -= 1;
			},
			AttackType::Hitscan(attributes) => todo!()
		}
	}

	pub fn should_rm(&self) -> bool {
		if self.lifetime == 0 {
			return true;
		}
		return false;
	}
}
