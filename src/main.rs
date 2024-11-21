use gameplay::gameplay;
use macroquad::window::next_frame;

mod gameplay;

/// Used to determine what state the game is in.
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
