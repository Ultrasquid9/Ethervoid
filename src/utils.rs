use parking_lot::RwLock;

use macroquad::{
	input::mouse_position,
	math::Vec2,
	time::get_frame_time,
	window::{screen_height, screen_width},
};

pub mod config;
pub mod error;
pub mod resources;

// Stores the delta time of the given frame.
static DELTA_TIME: RwLock<f32> = RwLock::new(0.);

pub fn point_to_vec2(point: std::boxed::Box<(dyn raywoke::point::Point + 'static)>) -> Vec2 {
	let (x, y) = (point.x(), point.y());
	Vec2::new(x, y)
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> Vec2 {
	let (x, y) = mouse_position();

	Vec2::new(x, y) - Vec2::new(screen_width() / 2., screen_height() / 2.)
}

/**
Gets the current delta time, and stores it.

This is done because Macroquad's `get_frame_time()` function panics in multithreaded scenarios.
 */
pub fn update_delta_time() {
	*DELTA_TIME.write() = get_frame_time() * 100. * (2. / 3.)
}

/// Gets the delta time
pub fn get_delta_time() -> f32 {
	*DELTA_TIME.read()
}

/// Gets the scale that the camera should be rendered at
pub fn camera_scale() -> f32 {
	screen_width() / screen_height() * 512.
}
