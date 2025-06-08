use ahash::HashMap;
use raywoke::prelude::*;
use stecs::prelude::*;

use super::{
	Gameplay,
	ecs::{
		health::Health,
		obj::Obj,
		sprite::{Frames, Rotation, Sprite},
	},
	paused::Paused,
};

use crate::utils::{
	get_delta_time, get_mouse_pos, resources::textures::access_image, tup_vec::Tup64,
};

use rhai::{CustomType, TypeBuilder};

#[derive(Clone, SplitFields)]
pub struct Attack {
	pub obj: Obj,

	pub owner: Owner,
	pub is_parried: bool,

	pub atk_type: AttackType,
	damage: f64,
	lifetime: f64,

	pub sprite: Sprite,
}

#[derive(Clone, PartialEq)]
pub enum Owner {
	Player,
	Enemy,
}

#[derive(Clone, PartialEq)]
pub enum AttackType {
	Physical,
	Burst,
	Projectile,
	Hitscan,
}

impl Attack {
	pub fn new_physical(obj: Obj, damage: f64, owner: Owner, key: &str) -> Attack {
		Attack {
			obj,

			owner,
			is_parried: false,

			atk_type: AttackType::Physical,
			damage,
			lifetime: 2.,

			sprite: Sprite::new(
				obj,
				key,
				Rotation::Angle,
				Frames::new_attack(),
				HashMap::default(),
			),
		}
	}

	pub fn new_burst(obj: Obj, damage: f64, owner: Owner, key: &str) -> Attack {
		Attack {
			obj,

			owner,
			is_parried: false,

			atk_type: AttackType::Burst,
			damage,
			lifetime: 12.,

			sprite: Sprite::new(
				obj,
				key,
				Rotation::Static,
				Frames::new_attack(),
				HashMap::default(),
			),
		}
	}

	pub fn new_projectile(obj: Obj, damage: f64, owner: Owner, key: &str) -> Attack {
		Attack {
			obj: Obj::new(obj.pos, ((obj.target - obj.pos) * 999.) + obj.pos, obj.size),

			owner,
			is_parried: false,

			atk_type: AttackType::Projectile,
			damage,
			lifetime: 1.,

			sprite: Sprite::new(
				obj,
				key,
				Rotation::Static,
				Frames::new_static(),
				HashMap::default(),
			),
		}
	}

	pub fn new_hitscan(obj: Obj, damage: f64, owner: Owner) -> Attack {
		Attack {
			obj,

			owner,
			is_parried: false,

			atk_type: AttackType::Hitscan,
			damage,
			lifetime: 8.,

			sprite: Sprite::new(
				obj,
				"default:attacks/projectile-enemy",
				Rotation::Static,
				Frames::new_static(),
				HashMap::default(),
			),
		}
	}
}

// Allows Attacks to be created by scripts
impl CustomType for Attack {
	fn build(mut builder: TypeBuilder<Self>) {
		builder.with_name("attack");
	}
}

pub fn handle_combat(gameplay: &mut Gameplay) {
	try_parry(gameplay);

	for (_, mut atk) in gameplay.world.attacks.iter_mut() {
		atk.sprite.update(*atk.obj);

		// Handling the lifetime and movement of attacks
		if *atk.atk_type == AttackType::Projectile {
			let new_pos = atk
				.obj
				.pos
				.move_towards(atk.obj.target, get_delta_time() * 5.);
			atk.obj.try_move(&new_pos, &gameplay.current_map);

			if atk.obj.pos != new_pos {
				*atk.lifetime = 0.;
			}
		} else {
			*atk.lifetime -= get_delta_time();
		}

		let func = match atk.atk_type {
			AttackType::Physical => attack_physical,
			AttackType::Burst => attack_burst,
			AttackType::Projectile => attack_projectile,
			AttackType::Hitscan => attack_hitscan,
		};

		macro_rules! attack {
			($field:expr) => {
				for (obj, hp, sprite) in query!($field, (&mut obj, &mut health, &mut sprite)) {
					func(obj, hp, sprite, &mut atk)
				}
			};
		}

		match atk.owner {
			Owner::Player => attack!(gameplay.world.enemies),
			Owner::Enemy => attack!(gameplay.world.player),
		}
	}
}

fn attack_physical(obj: &mut Obj, hp: &mut Health, sprite: &mut Sprite, atk: &mut AttackRefMut) {
	if *atk.lifetime >= 0. && obj.is_touching(atk.obj) {
		hp.damage(*atk.damage);
		sprite.shake();

		if *atk.is_parried {
			obj.stunned = 40.;
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
		&Ray::new(atk.obj.pos.tup64(), atk.obj.target.tup64()),
		&obj.to_barriers(),
	)
	.is_ok()
	{
		sprite.shake();
		hp.damage(*atk.damage);
	}
}

/// Attempts to parry attacks
fn try_parry(gameplay: &mut Gameplay) {
	let attack_ids: Vec<usize> = gameplay.world.attacks.ids().collect();

	for i in attack_ids.iter().rev() {
		let atk_1 = gameplay.world.attacks.get(*i).expect("Attack should exist");

		if *atk_1.atk_type != AttackType::Physical || *atk_1.is_parried {
			continue;
		}

		for j in attack_ids.iter().rev() {
			if *i == *j {
				continue;
			}

			let atk_2 = gameplay.world.attacks.get(*j).expect("Attack should exist");

			if !atk_2.obj.is_touching(atk_1.obj) || *atk_2.is_parried {
				continue;
			}

			match atk_2.atk_type {
				// Physical attacks should not be able to parry themselves
				AttackType::Physical => {
					if atk_1.owner == atk_2.owner {
						continue;
					}
				}
				// Burst and hitscan attacks cannot be parried
				AttackType::Burst | AttackType::Hitscan => continue,

				AttackType::Projectile => (),
			}

			// I have no clue why the borrow checker approved of
			// the following code.
			//
			// I know its safe, but the borrow checker shouldn't.

			gameplay.paused = Paused::Hitstop(16.);

			let atk_1 = &mut gameplay.world.attacks.get_mut(*i).unwrap();
			*atk_1.lifetime += get_delta_time();
			*atk_1.is_parried = true;

			let new_owner = atk_1.owner.clone();
			let new_damage = *atk_1.damage;
			let new_target = atk_1.obj.target;

			let atk_2 = &mut gameplay.world.attacks.get_mut(*j).unwrap();
			*atk_2.owner = new_owner;
			*atk_2.damage += new_damage;
			*atk_2.is_parried = true;

			// Yes, I used two match blocks.
			// Unfortunately, this was needed because of borrow checker shenanigans.
			match atk_2.atk_type {
				AttackType::Physical => *atk_2.lifetime += get_delta_time(),

				AttackType::Projectile => {
					*atk_2.lifetime = 6.;
					*atk_2.atk_type = AttackType::Hitscan;

					atk_2
						.sprite
						.set_img(access_image("default:attacks/hitscan-enemy").clone());
					atk_2.obj.target = 999.
						* match atk_2.owner {
							Owner::Player => get_mouse_pos(),
							Owner::Enemy => new_target,
						};
				}

				_ => unreachable!("How did a non-parryable attack end up here?"),
			}

			break;
		}
	}
}
