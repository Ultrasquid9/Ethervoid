use crate::cores::enemytype::EnemyType;
use macroquad::math::DVec2;
use stecs::prelude::*;

use super::ecs::{
	behavior::{Behavior, goal::GoalBehavior},
	health::Health,
	obj::Obj,
	sprite::{Frames, Rotation, Sprite},
};

#[derive(SplitFields)]
pub struct Enemy {
	health: Health,
	obj: Obj,
	behavior: Behavior,
	pub sprite: Sprite,
}

impl Enemy {
	pub fn from_type(enemytype: &EnemyType, pos: &DVec2) -> Self {
		let obj = Obj::new(*pos, *pos, enemytype.size);

		Self {
			health: Health::new(enemytype.max_health),
			obj,
			behavior: Behavior::Goal(GoalBehavior::from_scripts(&enemytype.goals)),
			sprite: Sprite::new(
				obj,
				&enemytype.sprite,
				Rotation::EightWay,
				Frames::new_entity(),
				enemytype.anims.clone(),
			),
		}
	}
}
