use macroquad::math::Vec2;
use rhai::{CustomType, TypeBuilder};
use stecs::prelude::*;

use crate::utils::get_delta_time;

use super::ecs::{behavior::Behavior, obj::Obj, sprite::Sprite};

#[derive(SplitFields, Clone)]
pub struct Attack {
	pub obj: Obj,
	pub sprite: Sprite,

	pub attack_type: AttackType,
	pub owner: Owner,

	pub damage: f32,
	pub lifetime: f32
}

#[derive(PartialEq, Clone)]
pub enum AttackType {
	Physical,
	Burst,
	Projectile(Behavior<'static>),
	Hitscan
}

#[derive(PartialEq, Clone)]
pub enum Owner {
	Player,
	Enemy
}

impl Attack {
	pub fn new_attack(
		pos: Vec2,
		target: Vec2,
		_sprite: &str,
		attack_type: AttackType,
		damage: f32, 
		size: f32,
		owner: Owner
	) -> Attack {
		let obj = Obj::new(pos, target, size);

		Attack {
			obj,
			sprite: Sprite::new(&obj),

			attack_type,
			
			owner,

			damage,
			lifetime: 2.
		}
	}

	pub fn update(&mut self) {
		if let AttackType::Projectile(_) = self.attack_type {

		} else {
			self.lifetime -= get_delta_time()
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
