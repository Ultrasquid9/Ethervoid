use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};

use super::vec2_to_tuple;

/// Trait for an object that has a size and can be moved
pub trait MovableObj {
	fn get_size(&self) -> &f32;
	fn get_pos(&self) -> Vec2;
	fn edit_pos(&mut self) -> &mut Vec2;

	/// Attempts to move the object to the provided Vec2
	fn try_move(&mut self, target: Vec2, map: &Vec<Vec2>) {
		match cast_wide(
			&Ray {
				position: (self.get_pos().x, self.get_pos().y),
				end_position: (target.x, self.get_pos().y)
			}, 
			&create_barriers(map)
		) {
			Ok(_) => (),
			_ => self.edit_pos().x = target.x
		}
	
		match cast_wide(
			&Ray {
				position: (self.get_pos().x, self.get_pos().y),
				end_position: (self.get_pos().x, target.y)
			}, 
			&create_barriers(map)
		) {
			Ok(_) => (),
			_ => self.edit_pos().y = target.y
		}
	}

	/// Checks if the object is touching another object
	fn is_touching(&mut self, other: &mut dyn MovableObj,) -> bool {
		if self.get_pos().distance(*other.edit_pos()) <= *self.get_size() + *other.get_size() {
			return true;
		} else {
			return false;
		}
	}
}


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

// Allows the entity to be moved 
impl MovableObj for Entity {
	fn get_size(&self) -> &f32 {
		&self.size
	}

	fn get_pos(&self) -> Vec2 {
		return self.pos
	}

	fn edit_pos(&mut self) -> &mut Vec2 {
		&mut self.pos
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
