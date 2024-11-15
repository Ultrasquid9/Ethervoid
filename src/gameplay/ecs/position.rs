use std::sync::RwLock;

use macroquad::math::Vec2;
use once_cell::sync::Lazy;
use raylite::{cast, cast_wide, Barrier, Ray};

use crate::gameplay::{cores::map::Map, player::Axis, tuple_to_vec2, vec2_to_tuple};

// For keeping track of the recursion in `try_move`
static DEPTH: Lazy<RwLock<u8>> = Lazy::new(|| RwLock::new(0));

/// An object that can be moved 
pub struct MovableObj {
	pub pos: Vec2,
	pub target: Vec2,

	axis_horizontal: Axis,
	axis_vertical: Axis,

	pub size: f32
}

impl MovableObj {
	/// Creates a new MovableObj
	pub fn new(pos: Vec2, size: f32) -> Self {
		Self {
			pos, 
			target: pos,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			size
		}
	}

	/// Sets the target and the angle
	pub fn set_target(&mut self, target: Vec2) {
		self.target = target;

		let x_val = (self.pos.x - target.x) / self.pos.distance(target);
		let y_val = (self.pos.y - target.y) / self.pos.distance(target);

		match x_val.round() as i8 {
			-1 => self.axis_horizontal = Axis::Positive,
			0 => self.axis_horizontal = Axis::None,
			1 => self.axis_horizontal = Axis::Negative,
			_ => ()
		}

		match y_val.round() as i8 {
			-1 => self.axis_vertical = Axis::Positive,
			0 => self.axis_vertical = Axis::None,
			1 => self.axis_vertical = Axis::Negative,
			_ => ()
		}
	}

	/// Attempts to move the object to the provided Vec2
	pub fn try_move(&mut self, target: Vec2, map: &Map) {
		let mut barriers = create_barriers(&map.points);
		for i in &map.doors {
			barriers.push(i.to_barrier())
		}

		let old_pos = self.pos;
		let mut try_slope_movement = false;

		match cast_wide(
			&Ray {
				position: (self.pos.x, self.pos.y),
				end_position: (target.x, self.pos.y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.x = target.x
		}
	
		match cast_wide(
			&Ray {
				position: (self.pos.x, self.pos.y),
				end_position: (self.pos.x, target.y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.y = target.y
		}

		// Everything beyond this point is for handling slopes
		if !try_slope_movement { return }

		// Checking recursion
		if *DEPTH.read().unwrap() > 1 {
			*DEPTH.write().unwrap() = 0;
			return 
		} else {
			*DEPTH.write().unwrap() += 1
		}

		let mut wall_to_check = Barrier {
			// Rust assumes that this variable could possibly be uninitialized,
			// so I have to set a burner value that is never read. 
			positions: ((0., 0.), (0., 0.)) 
		};

		for i in barriers {
			match cast(
				&Ray {
					position: (self.pos.x, self.pos.y),
					end_position: (target.x, target.y)
				}, 
				&i
			) {
				Ok(_) => (),
				_ => wall_to_check = i
			}
		}

		if wall_to_check.positions.0.0 != wall_to_check.positions.1.0
		&& wall_to_check.positions.0.1 != wall_to_check.positions.1.1 {
			return;
		}
		
		let angle = tuple_to_vec2(wall_to_check.positions.0).angle_between(tuple_to_vec2(wall_to_check.positions.1));

		let new_pos = Vec2::new(
			old_pos.distance(target) * angle.cos(), 
			old_pos.distance(target) * angle.cos()
		);

		if (old_pos + new_pos).distance(target) > old_pos.distance(target) {
			self.try_move(old_pos + new_pos, map);
		} else {
			self.try_move(old_pos - new_pos, map);
		}
	}

	/// Checks if the object is touching another object
	fn is_touching(&self, other: &MovableObj) -> bool {
		if self.pos.distance(other.pos) <= self.size + other.size {
			return true;
		} else {
			return false;
		}
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
