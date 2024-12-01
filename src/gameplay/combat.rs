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
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("attack");
	}
}

pub fn handle_combat(world: &mut World) {
	for (_, mut atk) in world.attacks.iter_mut() {
		atk.sprite.update(*atk.obj);

		// Handling the lifetime and movement of attacks 
		if *atk.attack_type != AttackType::Projectile {
			*atk.lifetime -= get_delta_time()
		} else {
			let new_pos = atk.obj.pos.move_towards(atk.obj.target, get_delta_time() * 5.);	
			atk.obj.try_move(new_pos);
		
			if atk.obj.pos != new_pos {
				*atk.lifetime = 0.;
			}
		}

		let func = match atk.attack_type {
			AttackType::Physical => attack_physical,
			AttackType::Burst => attack_burst,
			AttackType::Projectile => attack_projectile,
			AttackType::Hitscan => attack_hitscan
		};

		match atk.owner {
			Owner::Player => for (obj, hp) in query!(world.enemies, (&obj, &mut health)) {
				func(obj, hp, &mut atk)
			},
			Owner::Enemy => for (obj, hp) in query!(world.player, (&obj, &mut health)) {
				func(obj, hp, &mut atk)
			},
		}
	}
}

fn attack_physical(obj: &Obj, hp: &mut Health, atk: &mut AttackRefMut) {
	if *atk.lifetime >= 0. && obj.is_touching(&atk.obj) {
		hp.damage(*atk.damage);
	}
}

fn attack_burst(obj: &Obj, hp: &mut Health, atk: &mut AttackRefMut) {
	// Returns the attack but with double the size
	let double_size = |obj: &Obj| {
		let mut to_return = *obj;
		to_return.size *= 2.;

		return to_return;
	};

	if *atk.lifetime >= 0. && obj.is_touching(&double_size(&atk.obj)) {
		hp.damage(*atk.damage * (obj.pos.distance(atk.obj.pos) / (atk.obj.size * 2.)));
	}
}

fn attack_projectile(obj: &Obj, hp: &mut Health, atk: &mut AttackRefMut) {
	if obj.is_touching(&atk.obj) {
		hp.damage(*atk.damage);
		*atk.lifetime = 0.;
		return
	}
}

fn attack_hitscan(obj: &Obj, hp: &mut Health, atk: &mut AttackRefMut) {
	if cast_wide(
		&Ray{
			position: vec2_to_tuple(&atk.obj.pos), 
			end_position: vec2_to_tuple(&atk.obj.target)
		}, 
		&obj.to_barriers()
	).is_ok() { hp.damage(*atk.damage) }
}

/// Attempts to parry attacks 
pub fn try_parry(world: &mut World) {
	let attacks = &mut world.attacks;

	for i in 0..attacks.attack_type.len() {
		let atk_1 = attacks.get(i).unwrap();

		if *atk_1.attack_type != AttackType::Physical
		|| *atk_1.is_parried {
			continue;
		}

		for j in 0..attacks.attack_type.len() {
			if i == j { continue }

			let atk_2 = attacks.get(j).unwrap();

			if !atk_2.obj.is_touching(&atk_1.obj)
			|| *atk_2.is_parried {
				continue;
			}

			// I have no clue why the borrow checker approved of 
			// the code inside this match block.
			// 
			// I know its safe, but the borrow checker shouldn't.
			match atk_2.attack_type {

				// Physical attacks 
				AttackType::Physical => {
					if atk_1.owner == atk_2.owner { continue }

					let atk_1 = &mut attacks.get_mut(i).unwrap();
					*atk_1.lifetime += get_delta_time();
					*atk_1.is_parried = true;

					let new_owner = atk_1.owner.clone();
					let new_damage = atk_1.damage.clone();

					let atk_2 = &mut attacks.get_mut(j).unwrap();
					*atk_2.owner = new_owner;
					*atk_2.damage += new_damage;
					*atk_2.lifetime += get_delta_time();
					*atk_2.is_parried = true;

					world.hitstop = 10.;

					break;
				}

				// Projectile attacks
				AttackType::Projectile => {
					let new_owner = atk_1.owner.clone();
					let new_damage = atk_1.damage.clone();
					let new_target = atk_1.obj.target;

					let atk_2 = &mut attacks.get_mut(j).unwrap();
					*atk_2.owner = new_owner;
					*atk_2.damage += new_damage;
					*atk_2.lifetime = 6.;
					*atk_2.is_parried = true;

					*atk_2.attack_type = AttackType::Hitscan;

					atk_2.obj.target = match atk_2.owner {
						Owner::Player => get_mouse_pos() * 999.,
						Owner::Enemy => new_target * 999.
					};

					world.hitstop = 10.;

					break;
				}

				// Burst and hitscan attacks cannot be parried
				_ => ()
			}
		}
	}
}
