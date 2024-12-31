use ahash::HashMap;
use stecs::prelude::*;

use super::ecs::{
	sprite::{
		Frames, 
		Rotation, 
		Sprite
	}, 
	health::Health, 
	obj::Obj, 
	World
};

use crate::utils::{
	get_delta_time, 
	get_mouse_pos, 
	vec2_to_tuple
};

use raylite::{
	cast_wide, 
	Ray
};

use rhai::{
	CustomType, 
	TypeBuilder
};

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
		Attack {
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
				Frames::new_attack(),
				HashMap::default()
			)
		}
	}

	pub fn new_burst(obj: Obj, damage: f32, owner: Owner, key: &str) -> Attack {
		Attack {
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
				Frames::new_attack(),
				HashMap::default()
			)
		}
	}

	pub fn new_projectile(obj: Obj, damage: f32, owner: Owner, key: &str) -> Attack {
		Attack {
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
				Frames::new_static(),
				HashMap::default()
			)
		}
	}

	pub fn new_hitscan(obj: Obj, damage: f32, owner: Owner) -> Attack {
		Attack {
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
				Frames::new_static(),
				HashMap::default()
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
	try_parry(world);

	for (_, mut atk) in world.attacks.iter_mut() {
		atk.sprite.update(*atk.obj);

		// Handling the lifetime and movement of attacks 
		if *atk.attack_type != AttackType::Projectile {
			*atk.lifetime -= get_delta_time()
		} else {
			let new_pos = atk.obj.pos.move_towards(atk.obj.target, get_delta_time() * 5.);	
			atk.obj.try_move(new_pos, &world.current_map);
		
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
			Owner::Player => for (obj, hp, sprite) in query!(world.enemies, (&mut obj, &mut health, &mut sprite)) {
				func(obj, hp, sprite, &mut atk)
			},
			Owner::Enemy => for (obj, hp, sprite) in query!(world.player, (&mut obj, &mut health, &mut sprite)) {
				func(obj, hp, sprite, &mut atk)
			},
		}
	}
}

fn attack_physical(obj: &mut Obj, hp: &mut Health, sprite: &mut Sprite, atk: &mut AttackRefMut) {
	if *atk.lifetime >= 0. && obj.is_touching(atk.obj) {
		hp.damage(*atk.damage);
		sprite.shake();

		if *atk.is_parried {
			obj.stunned = 40.
		}
	}
}

fn attack_burst(obj: &mut Obj, hp: &mut Health, sprite: &mut Sprite, atk: &mut AttackRefMut) {
	// Returns the attack but with double the size
	let double_size = |obj: &Obj| {
		let mut to_return = *obj;
		to_return.size *= 2.;

		to_return
	};

	if *atk.lifetime >= 0. && obj.is_touching(&double_size(atk.obj)) {
		sprite.shake();
		hp.damage(*atk.damage * (obj.pos.distance(atk.obj.pos) / (atk.obj.size * 2.)));
	}
}

fn attack_projectile(obj: &mut Obj, hp: &mut Health, sprite: &mut Sprite, atk: &mut AttackRefMut) {
	if obj.is_touching(atk.obj) {
		sprite.shake();
		hp.damage(*atk.damage);
		*atk.lifetime = 0.;
	}
}

fn attack_hitscan(obj: &mut Obj, hp: &mut Health, sprite: &mut Sprite, atk: &mut AttackRefMut) {
	if cast_wide(
		&Ray{
			position: vec2_to_tuple(&atk.obj.pos), 
			end_position: vec2_to_tuple(&atk.obj.target)
		}, 
		&obj.to_barriers()
	).is_ok() { 
		sprite.shake();
		hp.damage(*atk.damage) 
	}
}

/// Attempts to parry attacks 
fn try_parry(world: &mut World) {
	let attack_ids: Vec<usize> = world.attacks.ids().collect();

	for i in attack_ids.iter().rev() {
		let atk_1 = world.attacks.get(*i).unwrap();

		if *atk_1.attack_type != AttackType::Physical
		|| *atk_1.is_parried {
			continue;
		}

		for j in attack_ids.iter().rev() {
			if *i == *j { continue }

			let atk_2 = world.attacks.get(*j).unwrap();

			if !atk_2.obj.is_touching(atk_1.obj)
			|| *atk_2.is_parried {
				continue;
			}

			match atk_2.attack_type {
				// Physical attacks should not be able to parry themselves
				AttackType::Physical => {
					if atk_1.owner == atk_2.owner { continue }
				}
				// Burst and hitscan attacks cannot be parried
				AttackType::Burst | AttackType::Hitscan => continue,

				_ => ()
			}

			// I have no clue why the borrow checker approved of 
			// the following code.
			// 
			// I know its safe, but the borrow checker shouldn't.

			world.hitstop = 10.;

			let atk_1 = &mut world.attacks.get_mut(*i).unwrap();
			*atk_1.lifetime += get_delta_time();
			*atk_1.is_parried = true;

			let new_owner = atk_1.owner.clone();
			let new_damage = *atk_1.damage;
			let new_target = atk_1.obj.target;

			let atk_2 = &mut world.attacks.get_mut(*j).unwrap();
			*atk_2.owner = new_owner;
			*atk_2.damage += new_damage;
			*atk_2.is_parried = true;

			// Yes, I used two match blocks.
			// Unfortunately, this was needed because of borrow checker shenanigans.
			match atk_2.attack_type {
				AttackType::Physical => *atk_2.lifetime += get_delta_time(),

				AttackType::Projectile => {
					*atk_2.lifetime = 6.;
					*atk_2.attack_type = AttackType::Hitscan;
					atk_2.obj.target = match atk_2.owner {
						Owner::Player => get_mouse_pos() * 999.,
						Owner::Enemy => new_target * 999.
					};
				}

				_ => panic!("How did a non-parryable attack end up here?")
			}

			break;
		}
	}
}
