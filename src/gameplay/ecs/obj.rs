use macroquad::math::DVec2;
use parking_lot::RwLock;
use rayon::prelude::*;
use raywoke::prelude::*;

use crate::utils::{point_to_vec2, resources::maps::access_map};

// For keeping track of the recursion in `try_move`
static DEPTH: RwLock<u8> = RwLock::new(0);

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
		if cast_wide(
			&Ray::new(self.pos.as_vec2(), new_pos.as_vec2()),
			&map.doors
				.par_iter()
				.map(|door| door.to_barrier())
				.collect::<Vec<Barrier>>(),
		)
		.is_ok()
		{
			return;
		}

		let mut ok_x = true;
		let mut ok_y = true;

		for wall in &map.walls {
			let check = |x: f64, y: f64| -> bool {
				cast_wide(&Ray::new(self.pos.as_vec2(), (x, y)), wall).is_ok()
			};

			if check(new_pos.x, self.pos.y) {
				ok_x = false
			}
			if check(self.pos.x, new_pos.y) {
				ok_y = false
			}
		}

		if ok_x {
			self.pos.x = new_pos.x
		}
		if ok_y {
			self.pos.y = new_pos.y
		}
		if ok_x && ok_y {
			return;
		}

		// Checking recursion
		if *DEPTH.read() > 0 {
			*DEPTH.write() = 0;
			return;
		} else {
			*DEPTH.write() += 1;
			self.try_handle_angle(new_pos, current_map);
		}
	}

	fn try_handle_angle(&mut self, new_pos: DVec2, current_map: &str) {
		let map = access_map(current_map);

		let mut to_check = Barrier::new(
			// Rust assumes that this variable could possibly be uninitialized,
			// so I have to set a burner value that is never read.
			(0., 0.),
			(0., 0.),
		);

		'out: for wall in &map.walls {
			for bar in wall {
				if cast(
					&Ray::new((self.pos.x, self.pos.y), (new_pos.x, new_pos.y)),
					bar,
				)
				.is_ok()
				{
					to_check = bar.clone();
					break 'out;
				}
			}
		}

		if to_check.0.x() == to_check.1.x() || to_check.0.y() == to_check.1.y() {
			return;
		}

		let point0 = point_to_vec2(to_check.0);
		let point1 = point_to_vec2(to_check.1);

		let angle0 = (point1.x - point0.x).atan2(point1.y - point0.y);
		let angle1 = (point0.x - point1.x).atan2(point0.y - point1.y);

		let check_dist =
			|angle: f64, pos: DVec2| -> f64 { (DVec2::from_angle(angle) + self.pos).distance(pos) };

		let angle = if check_dist(angle0, new_pos) < check_dist(angle1, self.target) {
			angle0
		} else {
			angle1
		};

		// Newer pos
		let new_pos = DVec2::from_angle(angle) * self.pos.distance(self.target);

		self.try_move(self.pos + new_pos, current_map);
	}
}
