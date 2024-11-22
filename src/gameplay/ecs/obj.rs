use std::sync::{LazyLock, RwLock};

use macroquad::math::Vec2;
use raylite::{cast, cast_wide, Barrier, Ray};

use crate::utils::{resources::maps::access_map, tuple_to_vec2};

// For keeping track of the recursion in `try_move`
static DEPTH: LazyLock<RwLock<u8>> = LazyLock::new(|| RwLock::new(0));

#[derive(PartialEq, Clone, Copy)]
pub enum Axis {
	Positive,
	Negative,
	None
}

#[derive(Clone, Copy)]
pub struct Obj {
	pub pos: Vec2,
	pub target: Vec2,
	pub speed: f32,

	pub axis_horizontal: Axis,
	pub axis_vertical: Axis,

	pub size: f32
}

impl Obj {
	pub fn new(pos: Vec2, target: Vec2, size: f32) -> Self {
		Self {
			pos,
			target,
			speed: 1.,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			size
		}
	}

	/// Updates the Obj's target and axis
	pub fn update(&mut self, new_target: Vec2) {
		self.target = new_target;

		let x_val = (self.pos.x - self.target.x) / self.pos.distance(self.target);
		let y_val = (self.pos.y - self.target.y) / self.pos.distance(self.target);
		
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

	/// Attempts to move the Obj to its current target
	pub fn try_move(&mut self) {
		let barriers = access_map("default:test").walls;
		
		// Instantly returns if about to hit a door 
/* 		if cast_wide(
			&Ray {
				position: vec2_to_tuple(&self.get_pos()),
				end_position: vec2_to_tuple(&target)
			},
			&map.doors
				.iter()
				.map(|door| return door.to_barrier())
				.collect()
		).is_ok() { return }		 */	

		let mut try_slope_movement = false;

		match cast_wide(
			&Ray {
				position: (self.pos.x, self.pos.y),
				end_position: (self.target.x, self.pos.y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.x = self.target.x
		}
	
		match cast_wide(
			&Ray {
				position: (self.pos.x, self.pos.y),
				end_position: (self.pos.x, self.target.y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.y = self.target.y
		}

		// Everything beyond this point is for handling slopes
		if !try_slope_movement { return }

		// Checking recursion
		if *DEPTH.read().unwrap() > 0 {
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
			if cast(
				&Ray {
					position: (self.pos.x, self.pos.y),
					end_position: (self.target.x, self.target.y)
				}, 
				&i
			).is_ok() {
				wall_to_check = i;
				break
			}
		}

		if wall_to_check.positions.0.0 == wall_to_check.positions.1.0
		|| wall_to_check.positions.0.1 == wall_to_check.positions.1.1 {
			return;
		}

		let point0 = tuple_to_vec2(wall_to_check.positions.0);
		let point1 = tuple_to_vec2(wall_to_check.positions.1);

		let angle0 = (point1.x - point0.x).atan2(point1.y - point0.y);
		let angle1 = (point0.x - point1.x).atan2(point0.y - point1.y);
		
		let angle = if (Vec2::from_angle(angle0) + self.pos).distance(self.target)
		< (Vec2::from_angle(angle1) + self.pos).distance(self.target) {
			angle0
		} else {
			angle1
		};

		let new_pos = Vec2::from_angle(angle) * self.pos.distance(self.target);

		self.target = self.pos + new_pos;
		self.try_move();
	}
}
