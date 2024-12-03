use stecs::prelude::*;

use super::{
	combat::Attack, 
	enemy::Enemy, 
	npc::Npc, 
	player::Player
};

use crate::utils::{
	resources::maps::access_map,
	config::Config
};

pub mod behavior;
pub mod health;
pub mod obj;
pub mod sprite;

pub struct World {
	pub player: StructOf<Vec<Player>>,
	pub enemies: StructOf<Vec<Enemy>>,
	pub npcs: StructOf<Vec<Npc>>,
	pub attacks: StructOf<Vec<Attack>>,

	pub current_map: String,
	pub config: Config,
	pub hitstop: f32
}

impl World {
	/// Populates the world with content from the current map, and clears old content if it exists
	pub fn populate(&mut self) {
		// Removing old stuff
	
		while !self.enemies.ids.is_empty() {
			self.enemies.remove(0);
		}
	
		while !self.npcs.ids.is_empty() {
			self.npcs.remove(0);
		}
	
		while !self.attacks.ids.is_empty() {
			self.attacks.remove(0);
		}
	
		// Adding new stuff
	
		for (enemy, pos) in access_map(&self.current_map).enemies.iter() {
			let _ = self.enemies.insert(Enemy::from_type(enemy, pos));
		}
	
		for (npc, pos) in access_map(&self.current_map).npcs.iter() {
			let _ = self.npcs.insert(Npc::from_type(npc, pos));
		}
	}
}
