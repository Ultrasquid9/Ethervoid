use std::time::SystemTime;
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
	// Removing old log
	let _ = std::fs::rename("./output.log", "./output.log.old");

	// Setting up the log 
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				humantime::format_rfc3339_seconds(SystemTime::now()),
				record.level(),
				record.target(),
				message
			))
		})
		.level(log::LevelFilter::Debug)
		.chain(std::io::stdout())
		.chain(fern::log_file("output.log").unwrap())
		.apply()
		.unwrap();

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
