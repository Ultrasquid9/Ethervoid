use macroquad::math::DVec2;

use crate::{
	data::config::{Config, keymap::Key},
	gameplay::ecs::obj::{Axis, Obj},
	utils::smart_time,
};

#[derive(PartialEq, Clone, Default)]
pub struct PlayerController {
	pub dash_cooldown: f64,
	pub is_dashing: bool,
}

impl PlayerController {
	/// Handles player controls
	pub fn control(&mut self, obj: &mut Obj, config: &Config, current_map: &str) {
		let mut new_pos = DVec2::ZERO; // The pos to be moved to

		if !self.is_dashing {
			switch_dir_from_input(config, obj);
		}
		let axis = |axis: &Axis, f: &mut f64| match axis {
			Axis::Positive => *f += 1.,
			Axis::Negative => *f -= 1.,
			Axis::None => (),
		};
		axis(&obj.axis_vertical, &mut new_pos.y);
		axis(&obj.axis_horizontal, &mut new_pos.x);

		// Dashing
		if config.keymap.dash.is_down() && self.dash_cooldown <= 0. && new_pos != DVec2::ZERO {
			obj.speed += 12.;
			self.dash_cooldown += 70.;
		} else if self.dash_cooldown > 0. {
			if self.dash_cooldown > 55. {
				self.is_dashing = true;
				obj.speed = 12.;
			} else {
				self.is_dashing = false;
			}
			self.dash_cooldown -= smart_time();
		}

		// Makes the player build up speed over time, rather than instantly starting at max speed
		if obj.speed < 3.5 && new_pos != DVec2::ZERO {
			obj.speed += obj.speed / 6.;
		}

		// Makes the player slow down if their speed is high
		if obj.speed > 4.5 {
			obj.speed /= 1.5;
		}

		if new_pos == DVec2::ZERO {
			obj.speed = 1.0;
		} else {
			obj.update((new_pos.normalize() * smart_time()) + obj.pos);
			obj.try_move(&obj.target.clone(), current_map);
		}
	}
}

fn switch_dir_from_input(config: &Config, obj: &mut Obj) {
	// Checks to see if both key1 and key2 are being held at the same time.
	// If they are, sets the direction of the axis based upon the most recently pressed key.
	// Otherwise, sets the direction of the axis based upon the currently pressed key.
	fn io(key1: &Key, key2: &Key, axis: &mut Axis) {
		if key1.is_down() && key2.is_down() {
			if key1.is_pressed() && *axis != Axis::Negative {
				*axis = Axis::Negative;
			}
			if key2.is_pressed() && *axis != Axis::Positive {
				*axis = Axis::Positive;
			}
		} else if key1.is_down() {
			*axis = Axis::Negative;
		} else if key2.is_down() {
			*axis = Axis::Positive;
		} else {
			*axis = Axis::None;
		}
	}

	io(
		&config.keymap.up,
		&config.keymap.down,
		&mut obj.axis_vertical,
	);
	io(
		&config.keymap.left,
		&config.keymap.right,
		&mut obj.axis_horizontal,
	);
}
