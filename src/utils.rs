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

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y);
}

/// Converts the inputted tuple of f32 into a Vec2
pub fn tuple_to_vec2(tup: (f32, f32)) -> Vec2 {
	return Vec2::new(tup.0, tup.1);
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> Vec2 {
	return tuple_to_vec2(mouse_position()) - Vec2::new(screen_width() / 2., screen_height() / 2.)
}

/// Gets the delta time
pub fn get_delta_time() -> f32 {
	return get_frame_time() * 100. * (2./3.)
}
