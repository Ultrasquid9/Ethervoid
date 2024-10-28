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
			lifetime: 1,
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

	/// Checks if the attack is a hitscan attack
	pub fn is_hitscan(&self) -> bool {
		match self.attack_type {
			AttackType::Hitscan(_) => true,
			_ => false
		}
	}

	/// Gets the target of the attack
	/// Panics if the attack is not a Projectile or Hitscan
	pub fn get_target(&self) -> Vec2 {
		match &self.attack_type {
			AttackType::Projectile(attributes) => attributes.target,
			AttackType::Hitscan(attributes) => attributes.target,
			_ => panic!("Attack does not have a target")
		}
	}

	/// Checks if the attack should be removed
	pub fn should_rm(&self) -> bool {
		if self.lifetime == 0 {
			return true;
		}
		return false;
	}

	// The following code is for updating attacks
	// Be warned: expect horrors beyond human comprehension

	/// Updates the attack based upon its type
	pub fn update(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Vec<Vec2>) {
		match &self.attack_type {
			AttackType::Physical => self.update_physical(enemies, player), 
			AttackType::Burst => self.attack_burst(enemies, player),
			AttackType::Projectile(attributes) => self.attack_projectile(enemies, player, map, attributes.clone()),
			AttackType::Hitscan(attributes) => self.attack_hitscan(enemies, player, attributes.clone()),
		}
	}

	/// Updates the provided Physical attack
	fn update_physical(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.is_touching(self.size) {
					i.stats.try_damage(self.damage);
				}
			}
			Owner::Enemy(_) => if player.stats.is_touching(self.size) {
				player.stats.try_damage(self.damage);
			}
		}

		self.lifetime -= 1;
	}

	/// Updates the provided Burst attack
	fn attack_burst(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.is_touching(self.size * 2.) {
					i.stats.try_damage(self.damage * (i.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
				}
			}
			Owner::Enemy(_) => if player.stats.is_touching(self.size * 2.) {
				player.stats.try_damage(self.damage * (player.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
			}
		}

		self.lifetime -= 1;
	}

	/// Updates the provided Projectile attack
	fn attack_projectile(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Vec<Vec2>, attributes: ProjectileOrHitscan) {
		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.get_pos().distance(self.pos) <= i.stats.size + self.size {
					i.stats.try_damage(self.damage);
					self.lifetime = 0;
					return;
				}
			}
			Owner::Enemy(_) => if player.stats.get_pos().distance(self.pos) <= player.stats.size + self.size {
				player.stats.try_damage(self.damage);
				self.lifetime = 0;
				return;
			}
		}

		let new_pos = self.pos.move_towards(attributes.target, 3.0);
		try_move(&mut self.pos, new_pos, map);

		if self.pos != new_pos || self.pos == attributes.target {
			self.lifetime = 0;
		}
	}

	/// Updates the provided Hitscan attack
	fn attack_hitscan(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, attributes: ProjectileOrHitscan) {
		match self.owner {
			Owner::Player => for i in enemies {
				self.damage_with_raycast(&mut i.stats, attributes.target);
			}
			Owner::Enemy(_) => self.damage_with_raycast(&mut player.stats, attributes.target),
		}

		self.lifetime -= 1;
	}

	/// Attempts to damage the provided entity with a raycast
	/// Should probably be refactored at some point
	fn damage_with_raycast(&self, entity: &mut Entity, target: Vec2) {
		match cast_wide(
			&Ray{
				position: vec2_to_tuple(&self.pos), 
				end_position: vec2_to_tuple(&target)
			}, 
			&entity_to_barriers(entity)
		) {
			Ok(_) => entity.try_damage(self.damage),
			_ => ()
		}
	}
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

/// Converts the provided entity into two barriers, a horizontal and a vertical one 
fn entity_to_barriers(entity: &Entity) -> Vec<Barrier> {
	vec![
		Barrier {
			positions: (
				(entity.x() + entity.size, entity.y()),
				(entity.x() - entity.size, entity.y())
			)
		},
		Barrier {
			positions: (
				(entity.x(), entity.y() + entity.size),
				(entity.x(), entity.y() - entity.size)
			)
		}
	]
}
