use fern::colors::ColoredLevelConfig;
use parking_lot::RwLock;

use macroquad::{
	input::mouse_position,
	math::DVec2,
	time::get_frame_time,
	window::{screen_height, screen_width},
};
use raywoke::point::Point;

pub mod config;
pub mod error;
pub mod resources;

// Stores the delta time of the given frame.
static DELTA_TIME: RwLock<f64> = RwLock::new(0.);

pub fn point_to_vec2(point: std::boxed::Box<(dyn raywoke::point::Point + 'static)>) -> DVec2 {
	let (x, y) = point.tup_f64();
	DVec2::new(x, y)
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> DVec2 {
	let (x, y) = mouse_position().tup_f64();

	DVec2::new(x, y) - DVec2::new(screen_width() as f64 / 2., screen_height() as f64 / 2.)
}

/**
Gets the current delta time, and stores it.

This is done because Macroquad's `get_frame_time()` function panics in multithreaded scenarios.
 */
pub fn update_delta_time() {
	*DELTA_TIME.write() = get_frame_time() as f64 * 100. * (2. / 3.)
}

/// Gets the delta time
pub fn get_delta_time() -> f64 {
	*DELTA_TIME.read()
}

/// Gets the scale that the camera should be rendered at
pub fn camera_scale() -> f64 {
	screen_width() as f64 / screen_height() as f64 * 512.
}

/// Initiates the logger
pub fn init_log() {
	// Renaming old log
	let _ = std::fs::rename("./output.log", "./output.log.old");

	// Coloring log messages
	let colors = ColoredLevelConfig::new().info(fern::colors::Color::Green);

	// Creating new log
	fern::Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"[{}] [{}] [{}] {}",
				jiff::Zoned::now()
					.datetime()
					.round(jiff::Unit::Millisecond)
					.unwrap(),
				colors.color(record.level()),
				record.target(),
				message
			))
		})
		.level(log::LevelFilter::Warn)
		.level_for("ethervoid", log::LevelFilter::Debug)
		.chain(std::io::stdout())
		.chain(fern::log_file("output.log").unwrap())
		.apply()
		.unwrap();
}
