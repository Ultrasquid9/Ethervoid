use macroquad::math::Vec2;
use stecs::prelude::*;

use crate::utils::config::Config;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj};

#[derive(SplitFields)]
pub struct Player {
	pub health: Health,
	pub obj: Obj,
	pub behavior: Behavior,

	pub config: Config
}

impl Player {
	pub fn new() -> Self {
		let pos = Vec2::new(0., 0.);

		Self {
			health: Health::new(100.),
			obj: Obj::new(pos, pos, 15.),
			behavior: Behavior::Player,

			config: Config::read("./config.ron")
		}
	}
}
