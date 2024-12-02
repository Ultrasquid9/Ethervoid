use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::cores::enemytype::EnemyType;

use super::ecs::{behavior::{Behavior, EnemyBehavior}, health::Health, obj::Obj, sprite::{Frames, Rotation, Sprite}};

#[derive(SplitFields)]
pub struct Enemy<'a> {
	health: Health,
	obj: Obj,
	behavior: Behavior<'a>,
	pub sprite: Sprite,
}

impl Enemy<'_> {
	pub fn from_type(enemytype: &EnemyType, pos: &Vec2) -> Self {
		let obj = Obj::new(*pos, *pos, 15.);

		Self {
			health: Health::new(enemytype.max_health),
			obj,
			behavior: Behavior::Enemy(EnemyBehavior {
				movement: enemytype.movement.clone().build(),

				attacks: enemytype.attacks
					.iter()
					.map(|attack| attack.clone().build())
					.collect(),

				attack_index: 0,
				attack_cooldown: 40.
			}),
			sprite: Sprite::new(
				obj, 
				enemytype.size as u32,
				&enemytype.sprite, 
				Rotation::EightWay,
				Frames::new_entity()
			)
		}
	}
}
