use std::sync::atomic::{AtomicU64, Ordering};

use macroquad::prelude::*;

pub mod config;
pub mod error;
pub mod logger;
pub mod lua;
pub mod resources;
pub mod tup_vec;

/// Immutable, heap-allocated slice
pub type ImmutVec<T> = Box<[T]>;

/// Stores delta time as bits within an [`AtomicU64`].
struct AtomicF64(AtomicU64);

impl AtomicF64 {
	const fn new() -> Self {
		Self(AtomicU64::new(0))
	}

	fn get(&self) -> f64 {
		f64::from_bits(self.0.load(Ordering::Relaxed))
	}

	fn set(&self, new: f64) {
		self.0.store(new.to_bits(), Ordering::Relaxed);
	}
}

// Stores the delta time of the given frame.
static DELTA_TIME: AtomicF64 = AtomicF64::new();
// Stores the mouse position of the given frame.
static MOUSE_POS: (AtomicF64, AtomicF64) = (AtomicF64::new(), AtomicF64::new());
// Stores the screen width/height of the given frame.
static SCREEN_SIZE: (AtomicF64, AtomicF64) = (AtomicF64::new(), AtomicF64::new());

/// Gets the current delta time, and stores it.
///
/// This is done because Macroquad's `get_frame_time()` function panics in multithreaded scenarios.
pub fn update_delta_time() {
	DELTA_TIME.set(f64::from(get_frame_time()));
}

/// Gets the current mouse position, and stores it.
///
/// This is done because Macroquad's `mouse_position()` function panics in multithreaded scenarios.
pub fn update_mouse_pos() {
	let (x, y) = mouse_position();

	MOUSE_POS.0.set(x as f64);
	MOUSE_POS.1.set(y as f64);
}

/// Gets the current screen size, and stores it.
///
/// This is done because Macroquad's `screen_width()` and `screen_height()` functions panic in multithreaded scenarios.
pub fn update_screen_size() {
	SCREEN_SIZE.0.set(screen_width() as f64);
	SCREEN_SIZE.1.set(screen_height() as f64);
}

/// Gets the delta time
pub fn delta_time() -> f64 {
	DELTA_TIME.get()
}

/// Gets the current position of the mouse
pub fn mouse_pos() -> DVec2 {
	let screen_size_halved = dvec2(screen_size().x / 2., screen_size().y / 2.);
	dvec2(MOUSE_POS.0.get(), MOUSE_POS.1.get()) - screen_size_halved
}

/// Gets the current position of the mouse in the range of [1; -1].
pub fn mouse_pos_local() -> DVec2 {
	let mouse_pos = dvec2(MOUSE_POS.0.get(), MOUSE_POS.1.get());
	let screen_size = screen_size();

	dvec2(mouse_pos.x / screen_size.x, mouse_pos.y / screen_size.y) * 2.0 - DVec2::ONE
}

/// Gets the current size of the screen
pub fn screen_size() -> DVec2 {
	dvec2(SCREEN_SIZE.0.get(), SCREEN_SIZE.1.get())
}

/// Gets the delta time and multiplies it to approximately equal `1` at 60 FPS
pub fn smart_time() -> f64 {
	const MULT: f64 = 100. * (2. / 3.);

	delta_time() * MULT
}

/// Gets the scale that the camera should be rendered at
pub fn camera_scale() -> f64 {
	screen_size().x / screen_size().y * 512.
}

/// Calculates the angle between two vectors
pub fn angle_between(p0: &DVec2, p1: &DVec2) -> f64 {
	(p1.y - p0.y).atan2(p1.x - p0.x)
}
