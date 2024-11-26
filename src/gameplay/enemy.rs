use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::cores::enemytype::EnemyType;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj, sprite::Sprite};

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
			behavior: Behavior::Script(enemytype.movement.clone().build()),
			sprite: Sprite::new(obj, &enemytype.sprite, super::ecs::sprite::SpriteType::EightWay)
		}
	}
}
