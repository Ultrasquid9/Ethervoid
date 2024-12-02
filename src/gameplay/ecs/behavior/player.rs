use macroquad::math::Vec2;

use crate::{gameplay::ecs::obj::{Axis, Obj}, utils::{config::Config, get_delta_time}};

use super::PlayerBehavior;

/// Handles player movement.
/// Returns true if the player attempted to move but didn't. 
pub fn player_behavior(
	obj: &mut Obj, 
	behavior: &mut PlayerBehavior, 
	config: &Config,
	current_map: &str
) {
	// Checks to see if both Up and Down are being held at the same time.
	// If they are, sets the direction to move based upon the most recently pressed key. 
	// Otherwise, sets the direction to move based upon the currently pressed key.
	if config.keymap.up.is_down()
	&& config.keymap.down.is_down() {
		if config.keymap.up.is_pressed()
		&& obj.axis_vertical != Axis::Negative {
			obj.axis_vertical = Axis::Negative;
		} 
		if config.keymap.down.is_pressed()
		&& obj.axis_vertical != Axis::Positive {
			obj.axis_vertical = Axis::Positive;
		} 
	} else if config.keymap.up.is_down() {
		obj.axis_vertical = Axis::Negative;
	} else if config.keymap.down.is_down() {
		obj.axis_vertical = Axis::Positive;
	} else {
		obj.axis_vertical = Axis::None;
	}

	// Checks to see if both Left and Right are being held at the same time.
	// If they are, sets the direction to move based upon the most recently pressed key. 
	// Otherwise, sets the direction to move based upon the currently pressed key.
	if config.keymap.left.is_down()
	&& config.keymap.right.is_down() {
		if config.keymap.left.is_pressed()
		&& obj.axis_vertical != Axis::Negative {
			obj.axis_horizontal = Axis::Negative;
		} 
		if config.keymap.right.is_pressed()
		&& obj.axis_vertical != Axis::Positive {
			obj.axis_horizontal = Axis::Positive;
		} 
	} else if config.keymap.left.is_down() {
		obj.axis_horizontal = Axis::Negative;
	} else if config.keymap.right.is_down() {
		obj.axis_horizontal = Axis::Positive;
	} else {
		obj.axis_horizontal = Axis::None;
	}

	let mut new_pos = Vec2::new(0., 0.); // The pos to be moved to 

	match obj.axis_vertical {
		Axis::Positive => new_pos.y += 1.,
		Axis::Negative => new_pos.y -= 1.,
		Axis::None => ()
	}
	match obj.axis_horizontal {
		Axis::Positive => new_pos.x += 1.,
		Axis::Negative => new_pos.x -= 1.,
		Axis::None => ()
	}

	// Dashing
	if config.keymap.dash.is_down() && behavior.dash_cooldown <= 0.{
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
	if behavior.speed < 3.5 && new_pos != Vec2::new(0., 0.) {
		behavior.speed += behavior.speed / 6.;
	}

	// Makes the player slow down if their speed is high
	if behavior.speed > 4.5 {
		behavior.speed /= 1.5;
	}

	if new_pos == Vec2::new(0., 0.) {
		behavior.speed = 1.0;
	} else {
		obj.update(new_pos.normalize() * behavior.speed * get_delta_time() + obj.pos);
		obj.try_move(obj.target, current_map);
	}
}
