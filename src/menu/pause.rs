use macroquad::prelude::*;

use crate::State;

use super::button;

/// The pause menu
pub async fn menu() -> Option<State> {
	let mut to_return: Option<State> = None;

	let y_pos = |height: f32| (screen_height() / 2.) + ((screen_height() / 10.) * height);

	if button("Resume", y_pos(-1.)) {
		to_return = Some(State::Gameplay);
	}
	if button("Main Menu", y_pos(0.)) {
		to_return = Some(State::Menu);
	}
	if button("Quit", y_pos(1.)) {
		to_return = Some(State::Quit);
	}

	to_return
}
