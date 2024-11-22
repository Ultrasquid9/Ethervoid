use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::cores::enemytype::EnemyType;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj};

#[derive(SplitFields)]
pub struct Enemy<'a> {
	health: Health,
	obj: Obj,
	behavior: Behavior<'a>
}

impl Enemy<'_> {
	pub fn from_type(enemytype: &EnemyType, pos: &Vec2) -> Self {
		Self {
			health: Health::new(enemytype.max_health),
			obj: Obj::new(*pos, *pos, enemytype.size),
			behavior: Behavior::Script(enemytype.movement.clone().build())
		}
	}
}
