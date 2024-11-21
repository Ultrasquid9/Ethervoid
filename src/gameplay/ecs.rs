use stecs::prelude::StructOf;

use super::{enemy::Enemy, player::Player};

pub mod behavior;
pub mod health;
pub mod obj;

pub struct World<'a> {
	pub player: StructOf<Vec<Player<'a>>>,
	pub enemies: StructOf<Vec<Enemy<'a>>>,
}
