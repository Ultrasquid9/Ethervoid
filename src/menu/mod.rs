use macroquad::{color::GRAY, math::vec2, ui::{root_ui, widgets}, window::{clear_background, next_frame, screen_height, screen_width}};

use crate::State;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None;

	// creating the UI style
	let label_style = root_ui()
		.style_builder()
		.font_size(64)
		.build();

	let skin = macroquad::ui::Skin {
		label_style,

		..root_ui().default_skin()
	};
	
	root_ui().push_skin(&skin);

	// The menu
	loop {
		clear_background(GRAY);

		// Play button
		if widgets::Button::new("Play")
			.position(vec2(screen_width() - 64., (screen_height() / 3.) + 0.))
			.size(vec2(screen_width() / 16., screen_height() / 32.))
			.ui(&mut *root_ui()) {
				to_return = Some(State::Gameplay)
			}

		// Quit button 
		if widgets::Button::new("Quit")
			.position(vec2(screen_width() - 64., (screen_height() / 3.) + 64.))
			.size(vec2(screen_width() / 16., screen_height() / 32.))
			.ui(&mut *root_ui()) {
				to_return = Some(State::Quit)
			}

		// Checking to see if a button was pressed
		match to_return {
			Some(state) => return state,
			_ => ()
		}

		next_frame().await
	}
}
