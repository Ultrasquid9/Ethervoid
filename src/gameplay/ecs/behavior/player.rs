use macroquad::math::Vec2;

use crate::{gameplay::ecs::obj::{Axis, Obj}, utils::{config::Config, get_delta_time}};

pub fn player_movement(obj: &mut Obj, config: &Config) {
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

	// Makes the player build up speed over time, rather than instantly starting at max speed
	if obj.speed < 3.5 && new_pos != Vec2::new(0., 0.) {
		obj.speed += obj.speed / 6.;
	}

	// Makes the player slow down if their speed is high
	if obj.speed > 4.5 {
		obj.speed /= 1.5;
	}

	if new_pos == Vec2::new(0., 0.) {
		obj.speed = 1.0;
	} else {
		obj.update(new_pos.normalize() * obj.speed * get_delta_time() + obj.pos);
		obj.try_move();
	}
}