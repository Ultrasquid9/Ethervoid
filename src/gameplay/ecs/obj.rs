use macroquad::math::DVec2;
use raywoke::prelude::*;

use crate::{
	gameplay::doors::Door,
	utils::{
		angle_between,
		resources::maps::access_map,
		tup_vec::{DV2, Tup64},
	},
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
	pub size: f64,

	pub axis_horizontal: Axis,
	pub axis_vertical: Axis,

	pub speed: f64,
	pub stunned: f64,

	pub depth: u8,
}

impl Obj {
	pub fn new(pos: DVec2, target: DVec2, size: f64) -> Self {
		Self {
			pos,
			target,
			size,

			..Default::default()
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
		let bars = &map.doors.iter().map(Door::to_barrier).collect::<Vec<_>>();
		if cast_wide(&Ray::new(self.tup64(), new_pos.tup64()), bars).is_ok() {
			return;
		}

		// Handling speed
		let new_pos = if self.depth == 0 {
			((*new_pos - self.pos) * self.speed) + self.pos
		} else if self.depth == 1 {
			((*new_pos - self.pos) / 2.) + self.pos
		} else {
			*new_pos
		};

		let mut ok = true;
		for wall in &map.walls {
			if cast_wide(&Ray::new(self.tup64(), new_pos.tup64()), wall).is_ok() {
				ok = false;
				break;
			}
		}

		if ok {
			self.pos = new_pos;
			self.depth = 0;
			return;
		}

		// Checking/handling recursion
		if self.depth > 1 {
			self.depth = 0;
		} else {
			self.depth += 1;
			self.try_handle_angle(&new_pos, current_map);
		}
	}

	fn try_handle_angle(&mut self, new_pos: &DVec2, current_map: &str) {
		let mut to_check = DEFAULT_BAR;

		for wall in &access_map(current_map).walls {
			for bar in wall {
				if cast(&Ray::new(self.tup64(), new_pos.tup64()), bar).is_ok() {
					to_check = bar.clone();
				}
			}
		}

		if to_check.0 == DEFAULT_BAR.0 && to_check.1 == DEFAULT_BAR.1 {
			return;
		}

		let p0 = to_check.0.dvec2();
		let p1 = to_check.1.dvec2();

		let dot = (*new_pos - self.pos).dot(angle_vec(&p1, &p0) - angle_vec(&p0, &p1));
		let target = if dot > 0. { p0 } else { p1 };
		let dist = self.pos.distance(*new_pos) * (dot.abs() / 8.) * 1.1;

		self.try_move(&self.pos.move_towards(target, dist), current_map);
	}
}

impl Tup64 for Obj {
	fn tup64(&self) -> (f64, f64) {
		self.pos.tup64()
	}
}

impl Default for Obj {
	fn default() -> Self {
		Self {
			pos: DVec2::ZERO,
			target: DVec2::ZERO,
			size: 0.,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			speed: 1.,
			stunned: 0.,

			depth: 0,
		}
	}
}

fn angle_vec(p0: &DVec2, p1: &DVec2) -> DVec2 {
	DVec2::from_angle(angle_between(p0, p1))
}
