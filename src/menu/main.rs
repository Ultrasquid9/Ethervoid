use macroquad::prelude::*;

use crate::{State, utils::resources::langs::access_lang};

use super::button;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	let label_play = access_lang("menu.main.button.play");
	let label_quit = access_lang("menu.main.button.quit");

	// The menu
	let y_pos = |height: f32| (screen_height() / 2.) + ((screen_height() / 10.) * height);
	loop {
		clear_background(GRAY);

		if button(label_play, y_pos(-1.)) {
			to_return = Some(State::Gameplay);
		}

		if button(label_quit, y_pos(0.)) {
			to_return = Some(State::Quit);
		}

		if let Some(state) = to_return {
			return state;
		}

		next_frame().await;
	}
}
