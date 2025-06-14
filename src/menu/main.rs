use macroquad::prelude::*;

use crate::{
	State,
	gameplay::draw::process::to_texture,
	menu::button_size,
	utils::resources::{langs::access_lang, textures::access_image},
};

use super::button;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	let label_play = access_lang("menu_main_button_play");
	let label_quit = access_lang("menu_main_button_quit");

	let img = access_image("default:titlescreen_bad");
	let width = img.width() as f32;
	let height = img.height() as f32;

	let texture = to_texture(img);

	// The menu
	let y_pos =
		|height: f32| (screen_height() / 2.222) + (button_size().y * height) + (12. * height);

	loop {
		clear_background(GRAY);
		render_texture_fullscreen(&texture, width, height);

		if button(&label_play, y_pos(0.)) {
			to_return = Some(State::Gameplay);
		}

		if button(&label_quit, y_pos(1.)) {
			to_return = Some(State::Quit);
		}

		if let Some(state) = to_return {
			return state;
		}

		next_frame().await;
	}
}

fn render_texture_fullscreen(texture: &Texture2D, width: f32, height: f32) {
	let scale_x: f32;
	let scale_y: f32;

	if screen_width() < screen_height() * (width / height) {
		let scale = screen_height();

		scale_x = scale * (width / height);
		scale_y = scale;
	} else {
		let scale = screen_width();

		scale_y = scale * (height / width);
		scale_x = scale;
	}

	draw_texture_ex(
		texture,
		0.,
		0.,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(scale_x, scale_y)),
			..Default::default()
		},
	);
}
