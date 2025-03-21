use macroquad::math::DVec2;
use raywoke::prelude::*;

use crate::utils::{resources::maps::access_map, tup_vec::Tup64};

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
	pub fn try_move(&mut self, new_pos: DVec2, current_map: &str) {
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

		let mut ok_x = true;
		let mut ok_y = true;

		for wall in &map.walls {
			let check = |x: f64, y: f64| cast_wide(&Ray::new(self.tup64(), (x, y)), wall).is_ok();

			if check(new_pos.x, self.pos.y) {
				ok_x = false;
			}
			if check(self.pos.x, new_pos.y) {
				ok_y = false;
			}
		}

		if ok_x {
			self.pos.x = new_pos.x;
		}
		if ok_y {
			self.pos.y = new_pos.y;
		}
		if ok_x.eq(&ok_y) {
			return;
		}

		// Checking recursion
		if self.depth > 1 {
			self.depth = 0;
			return;
		} else {
			self.depth += 1;
			self.try_handle_angle(new_pos, current_map);
		}
	}

	fn try_handle_angle(&mut self, new_pos: DVec2, current_map: &str) {
		let map = access_map(current_map);
		let mut to_check = Barrier::new((0., 0.), (0., 0.));

		for wall in &map.walls {
			for bar in wall {
				if cast(&Ray::new(self.tup64(), new_pos.tup64()), bar).is_ok() {
					to_check = bar.clone();
				}
			}
		}

		if to_check.0.x() == to_check.1.x() || to_check.0.y() == to_check.1.y() {
			return;
		}

		let point0 = to_check.0;
		let point1 = to_check.1;

		let angle0 = (point1.x() - point0.x()).atan2(point1.y() - point0.y());
		let angle1 = (point0.x() - point1.x()).atan2(point0.y() - point1.y());

		let check_dist =
			|angle: f64, pos: DVec2| (DVec2::from_angle(angle) + self.pos).distance(pos);

		let angle = if check_dist(angle0, new_pos) < check_dist(angle1, new_pos) {
			angle0
		} else {
			angle1
		};

		// Newer pos
		let new_pos = DVec2::from_angle(angle) * self.pos.distance(new_pos);

		self.try_move(self.pos + new_pos, current_map);
	}
}

impl Tup64 for Obj {
	fn tup64(self: &Self) -> (f64, f64) {
		self.pos.tup64()
	}
}
