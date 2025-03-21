use macroquad::math::DVec2;
use raywoke::prelude::*;

use crate::utils::{
	resources::maps::access_map,
	tup_vec::{DV2, Tup64},
};

const DEFAULT_BAR: Barrier = Barrier((0., 0.), (0., 0.));

#[derive(PartialEq, Clone, Copy)]
pub enum Axis {
	Positive,
	Negative,
	None,
}

#[derive(Clone, Copy)]
pub struct Obj {
	pub pos: DVec2,
	pub target: DVec2,

	pub stunned: f64,
	pub depth: u8,

	pub axis_horizontal: Axis,
	pub axis_vertical: Axis,

	pub size: f64,
}

impl Obj {
	pub fn new(pos: DVec2, target: DVec2, size: f64) -> Self {
		Self {
			pos,
			target,

			stunned: 0.,
			depth: 0,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			size,
		}
	}

	/// Updates the Obj's target and axis
	pub fn update(&mut self, new_target: DVec2) {
		let calc = |diff: f64, axis: &mut Axis| {
			let val = diff / self.pos.distance(new_target);

			match val.round() as i8 {
				-1 => *axis = Axis::Positive,
				1 => *axis = Axis::Negative,
				_ => *axis = Axis::None,
			}
		};

		calc(self.pos.x - new_target.x, &mut self.axis_horizontal);
		calc(self.pos.y - new_target.y, &mut self.axis_vertical);

		self.target = new_target;
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
	pub fn try_move(&mut self, new_pos: &DVec2, current_map: &str) {
		let map = access_map(current_map);

		// Instantly returns if about to hit a door
		let bars = &map
			.doors
			.iter()
			.map(|door| door.to_barrier())
			.collect::<Vec<_>>();
		if cast_wide(&Ray::new(self.tup64(), new_pos.tup64()), bars).is_ok() {
			return;
		}

		let mut ok = true;

		for wall in &map.walls {
			if cast_wide(&Ray::new(self.tup64(), new_pos.tup64()), wall).is_ok() {
				ok = false;
				break;
			}
		}

		if ok {
			self.pos = *new_pos;
			return;
		}

		// Checking/handling recursion
		if self.depth > 1 {
			self.depth = 0;
		} else {
			self.depth += 1;
			self.try_handle_angle(new_pos, current_map);
		}
	}

	fn try_handle_angle(&mut self, new_pos: &DVec2, current_map: &str) {
		let map = access_map(current_map);
		let mut to_check = DEFAULT_BAR;

		for wall in &map.walls {
			for bar in wall {
				if cast(&Ray::new(self.tup64(), new_pos.tup64()), bar).is_ok() {
					to_check = bar.clone();
				}
			}
		}

		if to_check.0 == DEFAULT_BAR.0 && to_check.1 == DEFAULT_BAR.1 {
			return;
		}

		let point0 = to_check.0.dvec2();
		let point1 = to_check.1.dvec2();

		fn atan2(p0: &DVec2, p1: &DVec2) -> f64 {
			(p1.y - p0.y).atan2(p1.x - p0.x)
		}

		let angle0 = atan2(&point1, &point0);
		let angle1 = atan2(&point0, &point1);
		let angle2 = atan2(&self.pos, new_pos);

		let target = if (angle0 - angle2).abs() > (angle1 - angle2).abs() {
			point1
		} else {
			point0
		};

		self.try_move(
			&self.pos.move_towards(target, self.pos.distance(*new_pos)),
			current_map,
		);
	}
}

impl Tup64 for Obj {
	fn tup64(&self) -> (f64, f64) {
		self.pos.tup64()
	}
}
