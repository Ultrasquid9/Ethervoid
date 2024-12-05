use macroquad::{
	prelude::*, 
	ui::*
};

use crate::State;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main 

	// Styling the UI
	let skin = make_skin(); // Warning: moving this directly into `.push_skin()` causes a borrow_mut error
	root_ui().push_skin(&skin);

	// The menu
	loop {
		clear_background(GRAY);

		// Play button			
		if button(
			"Play", 
			vec2(screen_width() * 0.9, (screen_height() / 3.) + 0.)
		) { to_return = Some(State::Gameplay) }

		// Quit button 
		if button(
			"Quit", 
			vec2(screen_width() * 0.9, (screen_height() / 3.) + 64.)
		) { to_return = Some(State::Quit) }

		if let Some(state) = to_return { return state }

		next_frame().await
	}
}

/// Creates a button
fn button(label: &str, pos: Vec2) -> bool {
	let mut longest_side = if screen_height() > screen_width() {
		screen_height()
	} else {
		screen_width()
	} / 16.;

	let shortest_side = if screen_height() < screen_width() {
		screen_height()
	} else {
		screen_width()
	} / 7.5;

	longest_side = longest_side.clamp(
		shortest_side / 1.5, 
		shortest_side
	);

	widgets::Button::new(label)
		.position(pos)
		.size(Vec2::new(longest_side, longest_side / 2.))
		.ui(&mut root_ui())
}

/// Creates a skin for the UI
fn make_skin() -> Skin {
	// Text styling 
	let label_style = root_ui()
		.style_builder()
		.font_size(64)
		.build();

	Skin {
		label_style,

		..root_ui().default_skin()
	}
}
