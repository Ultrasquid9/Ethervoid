use macroquad::math::DVec2;

use crate::{
	gameplay::ecs::obj::{Axis, Obj},
	utils::{
		config::{Config, Key},
		get_delta_time,
	},
};

const CENTER: DVec2 = DVec2::new(0., 0.);

#[derive(PartialEq, Clone)]
pub struct PlayerBehavior {
	pub speed: f64,

	pub dash_cooldown: f64,
	pub is_dashing: bool,
}

/// Handles player movement.
pub fn player_behavior(
	obj: &mut Obj,
	behavior: &mut PlayerBehavior,
	config: &Config,
	current_map: &str,
) {
	let mut new_pos = CENTER; // The pos to be moved to

	if !behavior.is_dashing {
		switch_dir_from_input(config, obj)
	}
	match obj.axis_vertical {
		Axis::Positive => new_pos.y += 1.,
		Axis::Negative => new_pos.y -= 1.,
		Axis::None => (),
	}
	match obj.axis_horizontal {
		Axis::Positive => new_pos.x += 1.,
		Axis::Negative => new_pos.x -= 1.,
		Axis::None => (),
	}

	// Dashing
	if config.keymap.dash.is_down() && behavior.dash_cooldown <= 0. && new_pos != CENTER {
		behavior.speed += 12.;
		behavior.dash_cooldown += 70.;
	} else if behavior.dash_cooldown > 0. {
		if behavior.dash_cooldown > 55. {
			behavior.is_dashing = true;
			behavior.speed = 12.;
		} else {
			behavior.is_dashing = false;
		}
		behavior.dash_cooldown -= get_delta_time();
	}

	// Makes the player build up speed over time, rather than instantly starting at max speed
	if behavior.speed < 3.5 && new_pos != CENTER {
		behavior.speed += behavior.speed / 6.;
	}

	// Makes the player slow down if their speed is high
	if behavior.speed > 4.5 {
		behavior.speed /= 1.5;
	}

	if new_pos == CENTER {
		behavior.speed = 1.0;
	} else {
		obj.update(new_pos.normalize() * behavior.speed * get_delta_time() + obj.pos);
		obj.try_move(obj.target, current_map);
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
