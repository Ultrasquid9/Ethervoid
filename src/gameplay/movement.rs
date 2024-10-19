use macroquad::math::Vec2;
use raylite::{cast_wide, Barrier, Ray};

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pos: Vec2,

	pub size: f32,
	pub health: isize
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

	/// Creates a new Entity
	pub fn new(pos: Vec2, size: f32, health: isize) -> Self {
		return Entity {
			pos,
			size,
			health
		}
	}

	/// Tries to move the entity to the provided Vec2
	pub fn try_move(&mut self, new_pos: Vec2, map: &Vec<Vec2>) {
		match cast_wide(
			&Ray {
				position: (self.x(), self.y()),
				end_position: (new_pos.x, self.y())
			}, 
			&create_barriers(map)
		) {
			Ok(_) => (),
			_ => self.pos.x = new_pos.x
		}

		match cast_wide(
			&Ray {
				position: (self.x(), self.y()),
				end_position: (self.x(), new_pos.y)
			}, 
			&create_barriers(map)
		) {
			Ok(_) => (),
			_ => self.pos.y = new_pos.y
		}
	}
}

fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y)
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
