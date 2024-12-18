use fern::colors::ColoredLevelConfig;
use gameplay::gameplay;
use menu::menu;
use macroquad::prelude::*;

mod cores;
mod gameplay;
mod menu;
mod utils;

/// Used to determine what state the game is in.
pub enum State {
	Menu, // The main-menu
	Gameplay, // In-game
	Quit // Exiting the gamej
}

#[macroquad::main("Ethervoid")]
async fn main() {
	log();

	let mut state = State::Menu;

    loop {
		state = match state {
			State::Menu => menu().await,
			State::Gameplay => gameplay().await,
			State::Quit => return
		};

		next_frame().await
	}
}

fn log() {
	// Renaming old log
	let _ = std::fs::rename("./output.log", "./output.log.old");

	// Coloring log messages
	let colors = ColoredLevelConfig::new();

	// Creating new log
	fern::Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				jiff::Zoned::now().round(jiff::Unit::Second).unwrap(),
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
