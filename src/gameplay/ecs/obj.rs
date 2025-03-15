use macroquad::math::Vec2;
use parking_lot::RwLock;
use rayon::prelude::*;
use raywoke::prelude::*;

use crate::utils::{point_to_vec2, resources::maps::access_map};

use std::sync::LazyLock;

// For keeping track of the recursion in `try_move`
static DEPTH: LazyLock<RwLock<u8>> = LazyLock::new(|| RwLock::new(0));

#[derive(PartialEq, Clone, Copy)]
pub enum Axis {
	Positive,
	Negative,
	None,
}

#[derive(Clone, Copy)]
pub struct Obj {
	pub pos: Vec2,
	pub target: Vec2,

	pub stunned: f32,

	pub axis_horizontal: Axis,
	pub axis_vertical: Axis,

	pub size: f32,
}

impl Obj {
	pub fn new(pos: Vec2, target: Vec2, size: f32) -> Self {
		Self {
			pos,
			target,

			stunned: 0.,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			size,
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
			_ => (),
		}

		match y_val.round() as i8 {
			-1 => self.axis_vertical = Axis::Positive,
			0 => self.axis_vertical = Axis::None,
			1 => self.axis_vertical = Axis::Negative,
			_ => (),
		}
	}

	/// Checks if the Obj is touching another Obj
	pub fn is_touching(&self, other: &Self) -> bool {
		self.pos.distance(other.pos) <= self.size + other.size
	}

	/// Converts the Obj into two barriers, a horizontal and a vertical one
	pub fn to_barriers(self) -> [Barrier; 2] {
		[
			Barrier::new(
				(self.pos.x + self.size, self.pos.y),
				(self.pos.x - self.size, self.pos.y),
			),
			Barrier::new(
				(self.pos.x, self.pos.y + self.size),
				(self.pos.x, self.pos.y - self.size),
			),
		]
	}

	/// Attempts to move the Obj to its current target
	pub fn try_move(&mut self, new_pos: Vec2, current_map: &str) {
		let map = access_map(current_map);

		// Instantly returns if about to hit a door
		if cast_wide(
			&Ray::new(self.pos, self.target),
			&map.doors
				.par_iter()
				.map(|door| door.to_barrier())
				.collect::<Vec<Barrier>>(),
		)
		.is_ok()
		{
			return;
		}

		let mut try_slope_movement = false;

		match cast_wide(
			&Ray::new((self.pos.x, self.pos.y), (new_pos.x, self.pos.y)),
			&map.walls,
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.x = new_pos.x,
		}

		match cast_wide(
			&Ray::new((self.pos.x, self.pos.y), (self.pos.x, new_pos.y)),
			&map.walls,
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.pos.y = new_pos.y,
		}

		// Everything beyond this point is for handling slopes
		if !try_slope_movement {
			return;
		}

		// Checking recursion
		if *DEPTH.read() > 0 {
			*DEPTH.write() = 0;
			return;
		} else {
			*DEPTH.write() += 1
		}

		let mut wall_to_check = Barrier::new(
			// Rust assumes that this variable could possibly be uninitialized,
			// so I have to set a burner value that is never read.
			(0., 0.),
			(0., 0.),
		);

		for i in map.walls.iter() {
			if cast(
				&Ray::new((self.pos.x, self.pos.y), (new_pos.x, new_pos.y)),
				&i,
			)
			.is_ok()
			{
				wall_to_check = i.clone();
				break;
			}
		}

		if wall_to_check.0.x() == wall_to_check.1.x() || wall_to_check.0.y() == wall_to_check.1.y()
		{
			return;
		}

		let point0 = point_to_vec2(wall_to_check.0);
		let point1 = point_to_vec2(wall_to_check.1);

		let angle0 = (point1.x - point0.x).atan2(point1.y - point0.y);
		let angle1 = (point0.x - point1.x).atan2(point0.y - point1.y);

		let angle = if (Vec2::from_angle(angle0) + self.pos).distance(new_pos)
			< (Vec2::from_angle(angle1) + self.pos).distance(self.target)
		{
			angle0
		} else {
			angle1
		};

		// Newer pos
		let new_pos = Vec2::from_angle(angle) * self.pos.distance(self.target);

		self.try_move(self.pos + new_pos, current_map);
	}
}
