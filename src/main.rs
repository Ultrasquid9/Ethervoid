use gameplay::gameplay;
use macroquad::window::next_frame;
use menu::menu;

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
