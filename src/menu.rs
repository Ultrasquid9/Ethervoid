use std::sync::LazyLock;
use futures::executor::block_on;
use crate::State;

use macroquad::{
	prelude::*, 
	ui::*
};

pub static FONT: LazyLock<Font> = LazyLock::new(|| block_on(load_ttf_font("assets/fonts/PixeloidMono.ttf")).unwrap());

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main 

	// Styling the UI
	let skin = make_skin().await; // Warning: moving this directly into `.push_skin()` causes a borrow_mut error
	root_ui().push_skin(&skin);

	// The menu
	loop {
		clear_background(GRAY);

		// Play button			
		if button(
			"Play", 
			vec2(screen_width() - 96., (screen_height() / 3.) + 0.)
		) { to_return = Some(State::Gameplay) }

		// Quit button 
		if button(
			"Quit", 
			vec2(screen_width() - 96., (screen_height() / 3.) + 64.)
		) { to_return = Some(State::Quit) }

		if let Some(state) = to_return { return state }

		next_frame().await
	}
}

/// Creates a button
fn button(label: &str, pos: Vec2) -> bool {
	widgets::Button::new(label)
		.position(pos)
		.size(Vec2::new(screen_height() / 12., screen_height() / 24.).max(Vec2::new(48., 24.)))
		.ui(&mut root_ui())
}

/// Creates a skin for the UI
async fn make_skin() -> Skin {
	// Text styling 
	let label_style = root_ui()
		.style_builder()
		.font_size(25)
		.with_font(&FONT).unwrap()
		.build();

	Skin {
		label_style,

		..root_ui().default_skin()
	}
}
