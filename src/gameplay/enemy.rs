use crate::cores::enemytype::EnemyType;
use macroquad::math::DVec2;
use stecs::prelude::*;

use super::ecs::{
	behavior::goal::Goals,
	health::Health,
	obj::Obj,
	sprite::{Frames, Rotation, Sprite},
};

#[derive(SplitFields)]
pub struct Enemy {
	health: Health,
	obj: Obj,
	goals: Goals,
	pub sprite: Sprite,
}

impl Enemy {
	pub fn from_type(enemytype: &EnemyType, pos: &DVec2) -> Self {
		let obj = Obj::new(*pos, *pos, enemytype.size);

		Self {
			health: Health::new(enemytype.max_health),
			obj,
			goals: Goals::from_scripts(&enemytype.goals),
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
