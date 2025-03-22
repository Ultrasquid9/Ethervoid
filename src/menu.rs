use std::sync::OnceLock;

use macroquad::{prelude::*, ui::*};

pub mod main;
pub mod pause;

pub static FONT: OnceLock<Font> = OnceLock::new();

pub async fn init_ui() {
	_ = FONT.set(
		load_ttf_font("assets/fonts/PixeloidMono.ttf")
			.await
			.expect("Font should exist"),
	);

	let skin = make_skin().await; // Warning: moving this directly into `.push_skin()` causes a borrow_mut error
	root_ui().push_skin(&skin);
}

/// Creates a button
pub fn button(label: &str, pos: f32) -> bool {
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
