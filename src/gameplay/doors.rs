use crate::utils::{resources::maps::access_map, vec2_to_tuple};
use macroquad::math::Vec2;

use serde::{
	Deserialize, 
	Serialize
};

use raylite::{
	cast, 
	Barrier, 
	Ray
};

use super::ecs::{behavior::Behavior, World};

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

		false
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
	pub fn try_change_map(&self, world: &mut World) {
		let player = world.player.get_mut(0).unwrap();

		let speed = if let Behavior::Player(behavior) = player.behavior {
			behavior.speed + 1.
		} else {
			panic!("If you are seeing this, the player does not have the player behavior. This is a huge problem. Fortunately, you should probably never see this.")
		};
		let mut new_pos = player.obj.pos + match self.direction {
			Direction::North => Vec2::new(0., -speed),
			Direction::South => Vec2::new(0., speed),
			Direction::East => Vec2::new(-speed, 0.),
			Direction::West => Vec2::new(speed, 0.)
		};

		let ray = Ray {
			position: vec2_to_tuple(&player.obj.pos),
			end_position: vec2_to_tuple(&new_pos)
		};

		// The player has not touched the door, so the map should not be changed
		if cast(&ray, &self.to_barrier()).is_err() {
			return
		}
		
		for i in access_map(&self.dest).doors.clone() {
			if i.dest != world.current_map { continue }

			if !i.direction.is_opposing(&self.direction) {
				panic!("Door in {} does not match direction of door in {}", world.current_map, self.dest)
			}

			new_pos += match self.direction {
				Direction::North => Vec2::new(0., -speed),
				Direction::South => Vec2::new(0., speed),
				Direction::East => Vec2::new(-speed, 0.),
				Direction::West => Vec2::new(speed, 0.)
			};
			player.obj.pos = new_pos - self.pos + i.pos;

			world.current_map = self.dest.clone();
			world.populate();
			return;
		}
	}
}
