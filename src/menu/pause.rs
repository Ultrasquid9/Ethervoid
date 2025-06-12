use macroquad::prelude::*;

use crate::{State, utils::resources::langs::access_lang};

use super::button;

/// The pause menu
pub fn menu() -> Option<State> {
	let mut to_return: Option<State> = None;

	let y_pos = |height: f32| (screen_height() / 2.) + ((screen_height() / 10.) * height);

	if button(&access_lang("menu_pause_button_resume"), y_pos(-1.)) {
		to_return = Some(State::Gameplay);
	}
	if button(&access_lang("menu_pause_button_main_menu"), y_pos(0.)) {
		to_return = Some(State::Menu);
	}
	if button(&access_lang("menu_pause_button_quit"), y_pos(1.)) {
		to_return = Some(State::Quit);
	}

	to_return
}
