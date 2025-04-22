use macroquad::prelude::*;
use raywoke::point::Point;
use tup_vec::DV2;

use unsafe_delta_time::UnsafeDeltaTime;

pub mod config;
pub mod error;
pub mod resources;
pub mod tup_vec;

// Stores the delta time of the given frame.
static DELTA_TIME: UnsafeDeltaTime = UnsafeDeltaTime::new();

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> DVec2 {
	let calc = |f: f32| f64::from(f) / 2.;
	mouse_position().tup_f64().dvec2() - dvec2(calc(screen_width()), calc(screen_height()))
}

/**
Gets the current delta time, and stores it.

This is done because Macroquad's `get_frame_time()` function panics in multithreaded scenarios.
 */
pub fn update_delta_time() {
	DELTA_TIME.set(f64::from(get_frame_time()) * 100. * (2. / 3.));
}

/// Gets the delta time
pub fn get_delta_time() -> f64 {
	DELTA_TIME.get()
}

/// Gets the scale that the camera should be rendered at
pub fn camera_scale() -> f64 {
	f64::from(screen_width()) / f64::from(screen_height()) * 512.
}

/// Initiates the logger. Should do nothing if already called.
pub async fn init_log() {
	// Renaming old log
	_ = std::fs::rename("./output.log", "./output.log.old");

	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_writer(logger::Logger::new)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
	info!("Logger created successfully");
}

mod logger {
    use std::{fs::File, io::{Stdout, Write}};

	pub struct Logger {
		stdout: Stdout,
		file: File
	}

	impl Logger {
		pub fn new() -> Self {
			let file = std::fs::OpenOptions::new()
				.append(true)
				.create(true)
				.open("./output.log")
				.expect("Could not create log file");

			let stdout = std::io::stdout();

			Self { stdout, file }
		}
	}

	impl Write for Logger {
		fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
			let ok_stdout = self.stdout.write(buf)?;
			let ok_file = self.file.write(buf)?;
			Ok(std::cmp::min(ok_stdout, ok_file))
		}

		fn flush(&mut self) -> std::io::Result<()> {
			self.stdout.flush()?;
			self.file.flush()?;
			Ok(())
		}
	}
}

/// A dangerous, hacky, and likely irrelevant optimization
mod unsafe_delta_time {
	use std::cell::UnsafeCell;

	pub struct UnsafeDeltaTime(UnsafeCell<f64>);

	unsafe impl Sync for UnsafeDeltaTime {}

	impl UnsafeDeltaTime {
		pub const fn new() -> Self {
			Self(UnsafeCell::new(0.))
		}

		/// Gets the stored value.
		///
		/// Should be safe, as no values are changed.
		pub fn get(&self) -> f64 {
			unsafe { *self.0.get() }
		}

		/// Sets the stored value.
		///
		/// Likely unsafe if called from multiple threads, so don't do that please.
		pub fn set(&self, new: f64) {
			unsafe { *self.0.get() = new }
		}
	}
}
