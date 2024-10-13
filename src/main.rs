mod input;
mod gameplay;

use gameplay::gameplay;
use macroquad::prelude::*;

enum State {
	Gameplay
}

#[macroquad::main("Ethervoid")]
async fn main() {
	let state = State::Gameplay;

    loop {
		match state {
			State::Gameplay => gameplay().await,
		}

        next_frame().await
    }
}
