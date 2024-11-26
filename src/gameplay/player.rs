use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::utils::config::Config;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj, sprite::Sprite};

#[derive(SplitFields)]
pub struct Player<'a> {
	pub health: Health,
	pub obj: Obj,
	pub behavior: Behavior<'a>,
	pub sprite: Sprite,

	pub config: Config
}

impl Player<'_> {
	pub fn new() -> Self {
		let pos = Vec2::new(0., 0.);
		let obj = Obj::new(pos, pos, 15.);

		Self {
			health: Health::new(100.),
			obj,
			behavior: Behavior::Player,
			sprite: Sprite::new(obj, "default:entity/player/player_spritesheet_wip", super::ecs::sprite::SpriteType::EightWay),

			config: Config::read("./config.ron")
		}
	}
}
