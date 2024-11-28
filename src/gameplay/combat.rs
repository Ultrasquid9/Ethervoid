use stecs::prelude::*;

use raylite::{
	cast_wide, 
	Ray
};

use rhai::{
	CustomType, 
	TypeBuilder
};

use crate::utils::{
	get_delta_time, get_mouse_pos, vec2_to_tuple
};

use super::ecs::{health::Health, obj::Obj, sprite::{Frames, Rotation, Sprite}, World};

#[derive(Clone, SplitFields)]
pub struct Attack {
	pub obj: Obj,

	pub owner: Owner,
	pub is_parried: bool,

	pub attack_type: AttackType,
	damage: f32,
	lifetime: f32,

	pub sprite: Sprite
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
	Projectile,
	Hitscan
}

impl Attack {
	pub fn new_physical(obj: Obj, damage: f32, owner: Owner, key: &str) -> Attack {
		return Attack {
			obj,

			owner,
			is_parried: false,

			attack_type: AttackType::Physical,
			damage,
			lifetime: 2.,

			sprite: Sprite::new(
				obj, 
				obj.size as u32, 
				key, 
				Rotation::Angle,
				Frames::new_attack()
			)
		}
	}

	pub fn new_burst(obj: Obj, damage: f32, owner: Owner, key: &str) -> Attack {
		return Attack {
			obj,

			owner,
			is_parried: false,

			attack_type: AttackType::Burst,
			damage,
			lifetime: 12.,

			sprite: Sprite::new(
				obj,
				obj.size as u32,
				key,
				Rotation::Static,
				Frames::new_attack()
			)
		}
	}

	pub fn new_projectile(obj: Obj, damage: f32, owner: Owner, key: &str) -> Attack {
		return Attack {
			obj: Obj::new(
				obj.pos, 
				((obj.target - obj.pos) * 999.) + obj.pos, 
				obj.size
			),

			owner,
			is_parried: false,

			attack_type: AttackType::Projectile,
			damage,
			lifetime: 1.,

			sprite: Sprite::new(
				obj,
				obj.size as u32,
				key,
				Rotation::Static,
				Frames::new_static()
			)
		}
	}

	pub fn new_hitscan(obj: Obj, damage: f32, owner: Owner) -> Attack {
		return Attack {
			obj,

			owner,
			is_parried: false,

			attack_type: AttackType::Hitscan,
			damage,
			lifetime: 8.,

			sprite: Sprite::new(
				obj,
				obj.size as u32,
				"default:attacks/projectile-enemy",
				Rotation::Static,
				Frames::new_static()
			)
		}
	}

	/// Checks if the attack should be removed
	pub fn should_rm(&self) -> bool {
		match self.attack_type {
			AttackType::Physical | AttackType::Burst => self.sprite.anim_completed(),
			_ => self.lifetime <= 0.
		}
	}

	// The following code is for updating attacks
	// Be warned: expect horrors beyond human comprehension

	/// Updates the attack based upon its type
	pub fn update(&mut self, world: &mut World) {
		self.sprite.update(self.obj);

		match &self.attack_type {
			AttackType::Physical => {
				if self.lifetime > 0. {
					self.attack_physical(world);
				}
			} 
			AttackType::Burst => {
				if self.lifetime > 0. {
					self.attack_burst(world);
				}
			}
			AttackType::Projectile => self.attack_projectile(world),
			AttackType::Hitscan => self.attack_hitscan(world),
		}
	}

	/// Updates the provided Physical attack
	fn attack_physical(&mut self, world: &mut World) {
		match self.owner {
			Owner::Player => for (obj, health) in query!(world.enemies, (&obj, &mut health)) {
				if obj.is_touching(&self.obj) {
					health.damage(self.damage);
				}
			}
			Owner::Enemy => for (obj, health) in query!(world.player, (&obj, &mut health)) {
				if obj.is_touching(&self.obj) {
					health.damage(self.damage);
				}
			}
		}

		self.lifetime -= get_delta_time();
	}

	/// Updates the provided Burst attack
	fn attack_burst(&mut self, world: &mut World) {
		// Returns the attack but with double the size
		let double_size = || {
			let mut to_return = self.clone();
			to_return.obj.size *= 2.;

			return to_return;
		};

		match self.owner {
			Owner::Player => for (obj, health) in query!(world.enemies, (&obj, &mut health)) {
				if obj.is_touching(&double_size().obj) {
					health.damage(self.damage * (obj.pos.distance(self.obj.pos) / (self.obj.size * 2.)));
				}
			}
			Owner::Enemy => for (obj, health) in query!(world.player, (&obj, &mut health)) {
				if obj.is_touching(&double_size().obj) {
					health.damage(self.damage * (obj.pos.distance(self.obj.pos) / (self.obj.size * 2.)));
				}
			}
		}

		self.lifetime -= get_delta_time();
	}

	/// Updates the provided Projectile attack
	fn attack_projectile(&mut self, world: &mut World) {
		match self.owner {
			Owner::Player => for (obj, health) in query!(world.enemies, (&obj, &mut health)) {
				if obj.is_touching(&self.obj) {
					health.damage(self.damage);
					self.lifetime = 0.;
					return
				}
			}
			Owner::Enemy => for (obj, health) in query!(world.player, (&obj, &mut health)) {
				if obj.is_touching(&self.obj) {
					health.damage(self.damage);
					self.lifetime = 0.;
					return
				}
			}
		}

		let new_pos = self.obj.pos.move_towards(self.obj.target, 5.);
		let new_pos = ((new_pos - self.obj.pos) * get_delta_time()) + self.obj.pos;
		
		self.obj.try_move(new_pos);

		if self.obj.pos != new_pos {
			self.lifetime = 0.;
		}
	}

	/// Updates the provided Hitscan attack
	fn attack_hitscan(&mut self, world: &mut World) {
		// Damages the provided entity with a raycast
		let damage_with_raycast = |obj: &Obj, hp: &mut Health| {
			if cast_wide(
				&Ray{
					position: vec2_to_tuple(&self.obj.pos), 
					end_position: vec2_to_tuple(&self.obj.target)
				}, 
				&obj.to_barriers()
			).is_ok() { hp.damage(self.damage) }
		};

		match self.owner {
			Owner::Player => for (obj, health) in query!(world.enemies, (&obj, &mut health)) {
				damage_with_raycast(obj, health)
			}
			Owner::Enemy => for (obj, health) in query!(world.player, (&obj, &mut health)) {
				damage_with_raycast(obj, health)
			}
		}

		self.lifetime -= get_delta_time();
	}
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

/// Attempts to parry attacks 
pub fn try_parry(attacks: &mut [Attack], world: &mut World) {
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

			// Coming up next: more nesting than the average bird

			// Checking if j is touching i and if j has not already been parried
			if attacks[j].obj.is_touching(&attacks[i].obj)
			&& !attacks[j].is_parried {
				// Checking the attack type of j
				match &attacks[j].attack_type {

					// Physical attacks
					AttackType::Physical => {
						if attacks[j].owner != attacks[i].owner {
							attacks[j].owner = attacks[i].owner.clone();

							attacks[j].damage += attacks[i].damage;
							
							attacks[i].lifetime += get_delta_time();
							attacks[j].lifetime += get_delta_time();

							attacks[i].is_parried = true;
							attacks[j].is_parried = true;

							world.hitstop = 10.;
						}
					}

					// Projectile attacks
					AttackType::Projectile => {
						if attacks[j].owner != attacks[i].owner {
							attacks[j].owner = attacks[i].owner.clone();
						}

						attacks[j].attack_type = AttackType::Hitscan;
						attacks[j].damage += attacks[i].damage;

						attacks[j].obj.target = match attacks[j].owner {
							Owner::Player => get_mouse_pos() * 999.,
							Owner::Enemy => attacks[i].obj.target * 999.
						};

						world.hitstop = 10.;

						// Since hitscan attacks cannot be parried, the is_parried bool is unneccessary
					}

					// Burst and Hitscan attacks cannot be parried
					_ => ()
				}
			}
		}
	}
}
