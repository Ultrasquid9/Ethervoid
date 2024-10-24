use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};
use rhai::{CustomType, TypeBuilder};

use super::{enemy::Enemy, entity::{try_move, Entity}, player::Player, vec2_to_tuple};

#[derive(Clone)]
pub struct Attack {
	pub size: f32,
	pub pos: Vec2,
	pub owner: Owner,

	attack_type: AttackType,
	damage: isize,
	lifetime: u8
}

#[derive(Clone)]
pub enum AttackType {
	Physical,
	Burst,
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

#[derive(Clone)]
pub enum Owner {
	Player,
	Enemy(usize)
}

#[derive(Clone)]
pub struct ProjectileOrHitscan {
	target: Vec2
}

impl Attack {
	pub fn new_physical(pos: Vec2, damage: isize, size: f32, owner: Owner) -> Attack {
		return Attack {
			size,
			pos,
			owner,

			attack_type: AttackType::Physical,
			damage,
			lifetime: 8,
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, size: f32, owner: Owner) -> Attack {
		return Attack {
			size,
			pos,
			owner,

			attack_type: AttackType::Burst,
			damage,
			lifetime: 12,
		}
	}

	pub fn new_projectile(pos: Vec2, target: Vec2, damage:isize, owner: Owner) -> Attack {
		return Attack {
			size: 10.,
			pos,
			owner,

			attack_type: AttackType::Projectile( ProjectileOrHitscan {
				target
			}),
			damage,
			lifetime: 1,
		}
	}

	pub fn new_hitscan(pos: Vec2, target: Vec2, damage: isize, owner: Owner) -> Attack {
		return Attack {
			size: 10.,
			pos,
			owner,

			attack_type: AttackType::Hitscan( ProjectileOrHitscan {
				target
			}),
			damage,
			lifetime: 8,
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

	pub fn update(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Vec<Vec2>) {
		match &self.attack_type {
			AttackType::Physical => {
				match self.owner {
					Owner::Player => {
						for i in enemies {
							self.attack_physical(&mut i.stats);
						}
					}
					Owner::Enemy(_) => self.attack_physical(&mut player.stats)	
				}

				self.lifetime -= 1;
			},
			AttackType::Burst => {
				match self.owner {
					Owner::Player => {
						for i in enemies {
							self.attack_burst(&mut i.stats);
						}
					}
					Owner::Enemy(_) => self.attack_burst(&mut player.stats)
				}

				self.lifetime -= 1;
			},
			AttackType::Projectile(attributes) => {
				// TODO: functionalize; decrease nesting
				match self.owner {
					Owner::Player => {
						for i in enemies {
							if i.stats.get_pos().distance(self.pos) <= i.stats.size + self.size {
								i.stats.health -= self.damage;
								self.lifetime = 0;
								return;
							}
						}
					}
					Owner::Enemy(_) => {
						if player.stats.get_pos().distance(self.pos) <= player.stats.size + self.size {
							player.stats.health -= self.damage;
							self.lifetime = 0;
							return;
						}
					}
				}

				let new_pos = self.pos.move_towards(attributes.target, 3.0);
				try_move(&mut self.pos, new_pos, map);

				if self.pos != new_pos || self.pos == attributes.target {
					self.lifetime = 0;
				}
			},
			AttackType::Hitscan(attributes) => {
				match self.owner {
					Owner::Player => {
						for i in enemies {
							self.attack_hitscan(&mut i.stats, attributes.target);
						}
					}
					Owner::Enemy(_) => self.attack_hitscan(&mut player.stats, attributes.target),
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

	/// Damages the provided entity with a physical attack
	fn attack_physical(&self, entity: &mut Entity) {
		if entity.get_pos().distance(self.pos) <= entity.size + self.size {
			entity.health -= self.damage;
		}
	}

	/// Damages the provided entity with a burst attack 
	fn attack_burst(&self, entity: &mut Entity) {
		if entity.get_pos().distance(self.pos) <= entity.size + (self.size * 2.) {
			entity.health -= self.damage * (entity.get_pos().distance(self.pos) / (self.size * 2.)) as isize;
		}
	}

	/// Damages the provided entity with a hitscan attack 
	fn attack_hitscan(&self, entity: &mut Entity, target: Vec2) {
		match cast_wide(
			&Ray{
				position: vec2_to_tuple(&self.pos), 
				end_position: vec2_to_tuple(&target)
			}, 
			&entity_to_barriers(entity)
		) {
			Ok(_) => entity.health -= self.damage,
			_ => ()
		}
	}
}

impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

/// Converts the provided entity into two barriers, a horizontal and a vertical one 
fn entity_to_barriers(enemy: &Entity) -> Vec<Barrier> {
	vec![
		Barrier {
			positions: (
				(enemy.x() + enemy.size, enemy.y()),
				(enemy.x() - enemy.size, enemy.y())
			)
		},
		Barrier {
			positions: (
				(enemy.x(), enemy.y() + enemy.size),
				(enemy.x(), enemy.y() - enemy.size)
			)
		}
	]
}