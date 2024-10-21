use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};

use super::{enemy::Enemy, entity::try_move, player::Player, vec2_to_tuple};

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

	pub fn is_hitscan(&self) -> bool {
		match self.attack_type {
			AttackType::Hitscan(_) => true,
			_ => false
		}
	}

	pub fn get_target(&self) -> Vec2 {
		match &self.attack_type {
			AttackType::Projectile(attributes) => attributes.target,
			AttackType::Hitscan(attributes) => attributes.target,
			_ => panic!("Attack does not have a target")
		}
	}

	pub fn update(&mut self, enemies: &mut Vec<Enemy>, _player: &Player, map: &Vec<Vec2>) {
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
				let new_pos = self.pos.move_towards(attributes.target, 3.0);

				for i in enemies {
					match cast_wide(
						&Ray{
							position: vec2_to_tuple(&self.pos), 
							end_position: vec2_to_tuple(&new_pos)
						}, 
						&enemy_to_barriers(i)
					) {
						Ok(_) => {
							i.stats.health -= self.damage;
							self.lifetime = 0;
							return;
						},
						_ => ()
					}
				}

				try_move(&mut self.pos, new_pos, map);

				if self.pos != new_pos || self.pos == attributes.target {
					self.lifetime = 0;
				}
			},
			AttackType::Hitscan(attributes) => {
				for i in enemies {
					match cast_wide(
						&Ray{
							position: vec2_to_tuple(&self.pos), 
							end_position: vec2_to_tuple(&attributes.target)
						}, 
						&enemy_to_barriers(i)
					) {
						Ok(_) => i.stats.health -= self.damage,
						_ => ()
					}
				}
				self.lifetime -= 1;
			}
		}
	}

	pub fn should_rm(&self) -> bool {
		if self.lifetime == 0 {
			return true;
		}
		return false;
	}
}

/// Converts the provided enemy into two barriers, a horizontal and a vertical one 
fn enemy_to_barriers(enemy: &Enemy) -> Vec<Barrier> {
	vec![
		Barrier {
			positions: (
				(enemy.stats.x() + enemy.stats.size, enemy.stats.y()),
				(enemy.stats.x() - enemy.stats.size, enemy.stats.y())
			)
		},
		Barrier {
			positions: (
				(enemy.stats.x(), enemy.stats.y() + enemy.stats.size),
				(enemy.stats.x(), enemy.stats.y() - enemy.stats.size)
			)
		}
	]
}