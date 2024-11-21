use macroquad::math::Vec2;
use stecs::prelude::*;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj};

#[derive(SplitFields)]
pub struct Player {
	health: Health,
	obj: Obj,
	behavior: Behavior
}

impl Player {
	pub fn new() -> Self {
		let pos = Vec2::new(0., 0.);

		Self {
			health: Health::new(100.),
			obj: Obj::new(pos, pos, 15.),
			behavior: Behavior::Player
		}
	}
}
