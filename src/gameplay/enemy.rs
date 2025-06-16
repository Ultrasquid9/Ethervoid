use crate::cores::{enemytype::EnemyType, script::Script};
use macroquad::math::DVec2;
use stecs::prelude::*;
use tracing::error;

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
			behavior: Behavior::Goal(GoalBehavior {
				goals: enemytype
					.goals
					.iter()
					.filter_map(|key| match Script::new(key) {
						Ok(ok) => Some(ok),
						Err(e) => {
							error!("Failed to eval script {key}: {e}");
							None
						}
					})
					.collect(),

				prev_goal: "none".to_owned(),

				index: None,
				err: None,
			}),
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
