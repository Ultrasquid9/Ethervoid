use stecs::prelude::*;

use super::ecs::{behavior::Behavior, obj::Obj};

#[derive(SplitFields)]
pub struct Attack {
	obj: Obj,
	attack_type: AttackType,

	damage: f32,
	lifetime: f32
}

pub enum AttackType {
	Physical,
	Burst,
	Projectile(Behavior),
	Hitscan
}
