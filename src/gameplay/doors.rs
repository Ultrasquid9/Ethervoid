use ahash::HashMap;
use macroquad::math::Vec2;
use raylite::{cast, Barrier, Ray};
use serde::{Deserialize, Serialize};
use stecs::prelude::Archetype;

use crate::gameplay::populate;

use super::{cores::map::Map, entity::MovableObj, player::Player, vec2_to_tuple, World};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Direction {
	North, 
	South,
	East, 
	West
}

impl Direction {
	/// Checks if the provided direction is opposite of the current one 
	fn is_opposing(&self, other: &Self) -> bool {
		let dirs = [self, other];

		if dirs.contains(&&Self::North) && dirs.contains(&&Self::South) {
			return true
		}
		if dirs.contains(&&Self::East) && dirs.contains(&&Self::West) {
			return true
		}

		return false
	}
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Door {
	direction: Direction,
	pos: Vec2,
	dest: String
}

impl Door {
	/// Converts the door into a barrier
	pub fn to_barrier(&self) -> Barrier {
		match self.direction {
			Direction::North | Direction::South => Barrier {
				positions: (
					(self.pos.x + 32., self.pos.y),
					(self.pos.x - 32., self.pos.y)
				)
			},
			Direction::East | Direction::West => Barrier {
				positions: (
					(self.pos.x, self.pos.y + 32.),
					(self.pos.x, self.pos.y - 32.)
				)
			}
		}
	}

	/// Checks if the map should be changed, and changes it if it should
	pub fn try_change_map(
		&self, 
		player: &mut Player, 
		new_pos: Vec2, 
		camera: &mut Vec2, 

		world: &mut World,

		current_map: &mut String, 
		maps: &HashMap<String, Map>
	) {
		let ray = Ray {
			position: vec2_to_tuple(&player.stats.get_pos()),
			end_position: vec2_to_tuple(&new_pos)
		};

		// The player has not touched the door, so the map should not be changed
		if let Err(_) = cast(&ray, &self.to_barrier()) {
			return
		}

		for i in &maps.get(&self.dest).unwrap().doors {
			if i.dest != *current_map { continue }

			if !i.direction.is_opposing(&self.direction) {
				panic!("Door in {current_map} does not match direction of door in {}", self.dest)
			}

			*player.stats.edit_pos() = new_pos - self.pos + i.pos;
			*camera = *camera - self.pos + i.pos;

			*current_map = self.dest.clone();

			// Removing attacks 
			let mut attack_ids: Vec<usize> = Vec::new();
			for i in world.attacks.ids() {
				attack_ids.push(i);
			}
			for i in attack_ids {
				world.enemies.remove(i);
			}

			populate(world, maps.get(current_map).unwrap());
		}
	}
}
