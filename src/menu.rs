use std::sync::OnceLock;

use macroquad::{prelude::*, ui::*};

use crate::utils::{error::EvoidResult, resources::config::access_config};

pub mod dialogue;
pub mod main;
pub mod pause;

pub static FONT: OnceLock<Font> = OnceLock::new();

pub async fn init_ui() -> EvoidResult<()> {
	_ = FONT.set(load_ttf_font("assets/fonts/PixeloidMono.ttf").await?);

	let skin = make_skin()?; // Warning: moving this directly into `.push_skin()` causes a borrow_mut error
	root_ui().push_skin(&skin);

	Ok(())
}

/// Creates a button
pub fn button(label: &str, pos: f32) -> bool {
	let size = button_size();
	let pos = vec2(screen_width() - size.x - 12., pos);

	widgets::Button::new(label)
		.position(pos)
		.size(size)
		.ui(&mut root_ui())
}

pub fn button_size() -> Vec2 {
	let screen_scale = access_config().screen_scale as f32;
	let screen_size = average_screen_size();

	vec2(
		(screen_size / 32.) * screen_scale,
		(screen_size / 48.) * screen_scale,
	)
}

pub fn average_screen_size() -> f32 {
	f32::midpoint(screen_width(), screen_height())
}

/// Creates a skin for the UI
fn make_skin() -> EvoidResult<Skin> {
	// Text styling
	let label_style = root_ui()
		.style_builder()
		.font_size(25)
		.with_font(FONT.get().expect("Font should exist"))?
		.build();

	Ok(Skin {
		label_style,

		..root_ui().default_skin()
	})
}
