use self::prelude::*;
use fern::colors::ColoredLevelConfig;
use gameplay::gameplay;

use menu::{menu, ui::init_ui};

mod cores;
mod gameplay;
mod menu;
mod utils;

/// Used to determine what state the game is in.
pub enum State {
	Menu,     // The main-menu
	Gameplay, // In-game
	Quit,     // Exiting the gamej
}

#[macroquad::main("Ethervoid")]
async fn main() {
	log();
	init_ui();

	let mut state = State::Menu;

	loop {
		state = match state {
			State::Menu => menu().await,
			State::Gameplay => gameplay().await,
			State::Quit => return,
		};

		next_frame().await
	}
}

fn log() {
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

pub mod prelude {
	pub use macroquad::prelude::*;
	pub use rayon::prelude::*;
	pub use stecs::prelude::*;

	pub use ahash::HashMap;

	pub use log::{debug, error, info, trace, warn};
}
