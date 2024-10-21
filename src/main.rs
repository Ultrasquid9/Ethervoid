mod input;
mod gameplay;

use gameplay::gameplay;
use macroquad::prelude::*;

/// Used to determine what state the game is in.
/// Eventually, this will hold the main menu and option screens.
pub enum State {
	Gameplay, // In-game
	Quit // Exiting the gamej
}

#[macroquad::main("Ethervoid")]
async fn main() {
	let mut state = State::Gameplay;

    loop {
		state = match state {
			State::Gameplay => gameplay().await,
			State::Quit => return
		};

        next_frame().await
    }
}
