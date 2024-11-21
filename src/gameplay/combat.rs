use stecs::prelude::*;

use super::ecs::{behavior::Behavior, obj::Obj};

#[derive(SplitFields)]
pub struct Attack<'a> {
	obj: Obj,
	attack_type: AttackType<'a>,

	damage: f32,
	lifetime: f32
}

pub enum AttackType<'a> {
	Physical,
	Burst,
	Projectile(Behavior<'a>),
	Hitscan
}
