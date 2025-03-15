use crate::cores::enemytype::EnemyType;
use macroquad::math::DVec2;
use rayon::prelude::*;
use stecs::prelude::*;

use super::ecs::{
	behavior::{Behavior, EnemyBehavior},
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
		let obj = Obj::new(*pos, *pos, 15.);

		Self {
			health: Health::new(enemytype.max_health),
			obj,
			behavior: Behavior::Enemy(EnemyBehavior {
				movement: enemytype.movement.clone().build(),

				attacks: enemytype
					.attacks
					.par_iter()
					.map(|attack| attack.clone().build())
					.collect(),

				attack_index: 0,
				attack_cooldown: 40.,

				err: None,
			}),
			sprite: Sprite::new(
				obj,
				enemytype.size as u32,
				&enemytype.sprite,
				Rotation::EightWay,
				Frames::new_entity(),
				enemytype.anims.clone(),
			),
		}
	}
}
