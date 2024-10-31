use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};
use rhai::{CustomType, TypeBuilder};

use super::{enemy::Enemy, entity::{Entity, MovableObj}, player::Player, vec2_to_tuple};

#[derive(Clone)]
pub struct Attack {
	pub size: f32,
	pub pos: Vec2,
	pub owner: Owner,
	pub is_parried: bool,

	attack_type: AttackType,
	damage: isize,
	lifetime: u8
}

#[derive(Clone, PartialEq)]
pub enum Owner {
	Player,
	Enemy
}

#[derive(Clone, PartialEq)]
pub enum AttackType {
	Physical,
	Burst,
	Projectile(ProjectileOrHitscan),
	Hitscan(ProjectileOrHitscan)
}

#[derive(Clone, PartialEq)]
pub struct ProjectileOrHitscan {
	target: Vec2
}

impl Attack {
	pub fn new_physical(pos: Vec2, damage: isize, size: f32, owner: Owner) -> Attack {
		return Attack {
			size,
			pos,
			owner,
			is_parried: false,

			attack_type: AttackType::Physical,
			damage,
			lifetime: 2,
		}
	}

	pub fn new_burst(pos: Vec2, damage: isize, size: f32, owner: Owner) -> Attack {
		return Attack {
			size,
			pos,
			owner,
			is_parried: false,

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
			is_parried: false,

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
			is_parried: false,

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
				if i.stats.is_touching(self) {
					i.stats.try_damage(self.damage);
				}
			}
			Owner::Enemy => if player.stats.is_touching(self) {
				player.stats.try_damage(self.damage);
			}
		}

		self.lifetime -= 1;
	}

	/// Updates the provided Burst attack
	fn attack_burst(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player) {
		// Returns the attack but with double the size
		let double_size = || {
			let mut to_return = self.clone();
			to_return.size *= 2.;

			return to_return;
		};

		match self.owner {
			Owner::Player => for i in enemies {
				if i.stats.is_touching(&mut double_size()) {
					i.stats.try_damage(self.damage * (i.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
				}
			}
			Owner::Enemy => if player.stats.is_touching(&mut double_size()) {
				player.stats.try_damage(self.damage * (player.stats.get_pos().distance(self.pos) / (self.size * 2.)) as isize);
			}
		}

		self.lifetime -= 1;
	}

	/// Updates the provided Projectile attack
	fn attack_projectile(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map: &Vec<Vec2>, attributes: ProjectileOrHitscan) {
		match self.owner {
			Owner::Player => for i in enemies {
				if self.is_touching(&i.stats) {
					i.stats.try_damage(self.damage);
					self.lifetime = 0;
					return;
				}
			}
			Owner::Enemy => if self.is_touching(&player.stats) {
				player.stats.try_damage(self.damage);
				self.lifetime = 0;
				return;
			}
		}

		let new_pos = self.pos.move_towards(attributes.target, 3.0);
		self.try_move(new_pos, map);

		if self.pos != new_pos || self.pos == attributes.target {
			self.lifetime = 0;
		}
	}

	/// Updates the provided Hitscan attack
	fn attack_hitscan(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, attributes: ProjectileOrHitscan) {
		// Damages the provided entity with a raycast
		let damage_with_raycast = |entity: &mut Entity| {
			match cast_wide(
				&Ray{
					position: vec2_to_tuple(&self.pos), 
					end_position: vec2_to_tuple(&attributes.target)
				}, 
				&entity_to_barriers(entity)
			) {
				Ok(_) => entity.try_damage(self.damage),
				_ => ()
			}
		};
		
		match self.owner {
			Owner::Player => enemies
				.iter_mut()
				.for_each(|i| damage_with_raycast(&mut i.stats)),
			Owner::Enemy => damage_with_raycast(&mut player.stats),
		}

		self.lifetime -= 1;
	}
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

// Allows Attacks to be moved
impl MovableObj for Attack {
	fn get_size(&self) -> &f32 {
		&self.size
	}

	fn get_pos(&self) -> Vec2 {
		return self.pos
	}

	fn edit_pos(&mut self) -> &mut Vec2 {
		&mut self.pos
	}
}

/// Attempts to parry attacks 
pub fn try_parry(attacks: &mut Vec<Attack>) {
	// i is the index of the attack that is trying to parry other attacks
	for i in (0..attacks.len()).rev() {
		// Checking if i is not physical or has been parried, and continuing the loop if either is true
		if attacks[i].attack_type != AttackType::Physical
		|| attacks[i].is_parried {
			continue;
		}

		// Looping through all the other attacks
		for j in (0..attacks.len()).rev() {
			if i == j {
				continue;
			}

			// Checking if j is touching i and if j has not already been parried
			if attacks[j].is_touching(&attacks[i])
			&& !attacks[j].is_parried {

				// Checking the attack type of j
				match attacks[j].attack_type {
					AttackType::Physical => {
						if attacks[j].owner != attacks[i].owner {
							attacks[j].owner = attacks[i].owner.clone();
							
							attacks[i].lifetime += 1;
							attacks[j].lifetime += 1;

							attacks[i].is_parried = true;
							attacks[j].is_parried = true;
						}
					}
					AttackType::Projectile(_) => {
						todo!()
					}
					// Burst and Hitscan attacks cannot be parried
					_ => ()
				}
			}
		}
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
