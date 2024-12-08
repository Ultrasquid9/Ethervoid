use std::sync::RwLock;

use macroquad::{
	window::{
		screen_height, 
		screen_width
	},
	input::mouse_position, 
	time::get_frame_time,
	math::Vec2
};

pub mod config;
pub mod resources;

// Stores the delta time of the given frame. 
static DELTA_TIME: RwLock<f32> = RwLock::new(0.);

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	(vec.x, vec.y)
}

/// Converts the inputted tuple of f32 into a Vec2
pub fn tuple_to_vec2(tup: (f32, f32)) -> Vec2 {
	Vec2::new(tup.0, tup.1)
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> Vec2 {
	tuple_to_vec2(mouse_position()) - Vec2::new(screen_width() / 2., screen_height() / 2.)
}

/**
Gets the current delta time, and stores it.

This is done because Macroquad's `get_frame_time()` function panics in multithreaded scenarios.
 */
pub fn update_delta_time() {
	*DELTA_TIME.write().unwrap() = get_frame_time() * 100. * (2./3.)
}

/// Gets the delta time
pub fn get_delta_time() -> f32 {
	*DELTA_TIME.read().unwrap()
}

/// Gets the scale that the camera should be rendered at
pub fn camera_scale() -> f32 {
	screen_width() / screen_height() * 512.
}
