use macroquad::prelude::*;

use crate::State;

use super::button;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	// The menu
	let y_pos = |height: f32| (screen_height() / 2.) + ((screen_height() / 10.) * height);
	loop {
		clear_background(GRAY);

		if button("Play", y_pos(-1.)) {
			to_return = Some(State::Gameplay);
		}

		if button("Quit", y_pos(0.)) {
			to_return = Some(State::Quit);
		}

		if let Some(state) = to_return {
			return state;
		}

		next_frame().await;
	}
}
