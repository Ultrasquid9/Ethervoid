#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use gameplay::gameplay;
use macroquad::prelude::*;
use menu::menu;

mod utils;
mod gameplay;
mod menu;

/// Used to determine what state the game is in.
/// Eventually, this will hold the main menu and option screens.
pub enum State {
	Menu, // The main-menu
	Gameplay, // In-game
	Quit // Exiting the gamej
}

#[macroquad::main("Ethervoid")]
async fn main() {
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
