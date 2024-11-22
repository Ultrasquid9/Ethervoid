use stecs::prelude::*;

use super::ecs::{behavior::Behavior, obj::Obj};

#[derive(SplitFields)]
pub struct Attack<'a> {
	pub obj: Obj,
	pub attack_type: AttackType<'a>,

	damage: f32,
	lifetime: f32
}

#[derive(PartialEq)]
pub enum AttackType<'a> {
	Physical,
	Burst,
	Projectile(Behavior<'a>),
	Hitscan
}
