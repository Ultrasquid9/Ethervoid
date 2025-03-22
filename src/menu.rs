use std::sync::OnceLock;

use crate::State;
use macroquad::{prelude::*, ui::*};

pub static FONT: OnceLock<Font> = OnceLock::new();

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	// Styling the UI
	_ = FONT.set(
		load_ttf_font("assets/fonts/PixeloidMono.ttf")
			.await
			.expect("Font should exist"),
	);

	let skin = make_skin().await; // Warning: moving this directly into `.push_skin()` causes a borrow_mut error
	root_ui().push_skin(&skin);

	// The menu
	let y_pos = |height: f32| (screen_height() / 2.) + ((screen_height() / 10.) * height);
	loop {
		clear_background(GRAY);

		if button("Play", y_pos(-1.)) {
			to_return = Some(State::Gameplay)
		}

		if button("Quit", y_pos(0.)) {
			to_return = Some(State::Quit)
		}

		if let Some(state) = to_return {
			return state;
		}

		next_frame().await
	}
}

/// Creates a button
fn button(label: &str, pos: f32) -> bool {
	let size = vec2(screen_height() / 12., screen_height() / 24.).max(vec2(48., 20.));
	let pos = vec2(screen_width() - size.x, pos);

	widgets::Button::new(label)
		.position(pos)
		.size(size)
		.ui(&mut root_ui())
}

/// Creates a skin for the UI
async fn make_skin() -> Skin {
	// Text styling
	let label_style = root_ui()
		.style_builder()
		.font_size(25)
		.with_font(FONT.get().expect("Font should exist"))
		.unwrap()
		.build();

	Skin {
		label_style,

		..root_ui().default_skin()
	}
}
