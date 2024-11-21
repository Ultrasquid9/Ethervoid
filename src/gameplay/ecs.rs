use stecs::prelude::StructOf;

use super::{enemy::Enemy, player::Player};

pub mod behavior;
pub mod health;
pub mod obj;

pub struct World {
	pub player: StructOf<Vec<Player>>,
	pub enemies: StructOf<Vec<Enemy>>,
}
