use stecs::prelude::*;

use super::{combat::Attack, enemy::Enemy, npc::Npc, player::Player};

use crate::utils::resources::maps::access_map;

pub mod behavior;
pub mod health;
pub mod obj;
pub mod sprite;

#[derive(Default)]
pub struct World {
	pub player: StructOf<Vec<Player>>,
	pub enemies: StructOf<Vec<Enemy>>,
	pub npcs: StructOf<Vec<Npc>>,
	pub attacks: StructOf<Vec<Attack>>,
}

impl World {
	/// Populates the world with content from the current map, and clears old content if it exists
	pub fn populate(&mut self, current_map: &str) {
		macro_rules! clear {
			( $( $field:expr ),+ ) => {
				$(
					while !$field.ids.is_empty() {
						$field.remove(0);
					}
				)+
			};
		}

		// Removing old stuff
		clear![self.enemies, self.npcs, self.attacks];

		// Adding new stuff
		for (enemy, pos) in &access_map(current_map).enemies {
			_ = self.enemies.insert(Enemy::from_type(enemy, pos));
		}
		for (npc, pos) in &access_map(current_map).npcs {
			_ = self.npcs.insert(Npc::from_type(npc, pos));
		}
	}
}
