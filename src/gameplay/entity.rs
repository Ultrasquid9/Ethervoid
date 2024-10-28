use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};

use super::vec2_to_tuple;

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pub i_frames: u8,
	pub size: f32,

	pos: Vec2,
	health: isize,
	id: Option<usize>,
}

impl Entity {
	/// Returns the x position
	pub fn x(&self) -> f32 {
		return self.pos.x;
	}

	/// Returns the y position
	pub fn y(&self) -> f32 {
		return self.pos.y;
	}

	/// Returns the position
	pub fn get_pos(&self) -> Vec2 {
		return self.pos;
	}

	/// Gets the health
	pub fn get_health(&self) -> isize {
		return self.health
	}

	/// Gets the ID
	pub fn get_id(&self) -> usize {
		match self.id {
			Some(id) => return id,
			_ => panic!("Tried to get ID of entity without ID. This was either a messed-up enemy or the player, either way this should never happen and if you're seeing this then something has gone very wrong")
		}
	}

	/// Creates a new Entity
	pub fn new(pos: Vec2, size: f32, health: isize, id: Option<usize>) -> Self {
		return Entity {
			i_frames: 0,
			pos,
			size,
			health,
			id
		}
	}
	
	/// Checks if the entity is touching another thing with the provided radius
	pub fn is_touching(&self, radius: f32) -> bool {
		if self.get_pos().distance(self.pos) <= radius + self.size {
			return true;
		} else {
			return false;
		}
	}

	/// Tries to move the entity to the provided Vec2
	pub fn try_move(&mut self, new_pos: Vec2, map: &Vec<Vec2>) {
		try_move(&mut self.pos, new_pos, map);
	}

	pub fn try_damage(&mut self, damage: isize) {
		if self.i_frames == 0 {
			self.health -= damage;
			self.i_frames = 12;
		}
	}

	/// Checks if the entity is dead
	pub fn should_kill(&self) -> bool {
		if self.health <= 0 {
			return true
		} else {
			return false
		}
	}
}

/// Tries to move the provided position to the provided target
pub fn try_move(pos: &mut Vec2, target: Vec2, map: &Vec<Vec2>) {
	match cast_wide(
		&Ray {
			position: (pos.x, pos.y),
			end_position: (target.x, pos.y)
		}, 
		&create_barriers(map)
	) {
		Ok(_) => (),
		_ => pos.x = target.x
	}

	match cast_wide(
		&Ray {
			position: (pos.x, pos.y),
			end_position: (pos.x, target.y)
		}, 
		&create_barriers(map)
	) {
		Ok(_) => (),
		_ => pos.y = target.y
	}
}
	
fn create_barriers(map: &Vec<Vec2>) -> Vec<Barrier> {
	let mut barriers: Vec<Barrier> = Vec::new();

	for i in 0..map.len() {
		match map.get(i + 1) {
			Some(_) => barriers.push(Barrier {
				positions: (vec2_to_tuple(map.get(i).unwrap()), vec2_to_tuple(map.get(i + 1).unwrap()))
			}),
			None => barriers.push(Barrier {
				positions: (vec2_to_tuple(map.get(i).unwrap()), (vec2_to_tuple(map.get(0).unwrap())))
			})
		}
	}

	return barriers;
}
