use macroquad::prelude::*;

use crate::{
	State,
	gameplay::draw::process::to_texture,
	menu::{average_screen_size, button_size},
	utils::resources::{langs::access_lang, textures::access_image},
};

use super::button;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	let label_play = access_lang("menu_main_button_play");
	let label_quit = access_lang("menu_main_button_quit");

	let titlescreen = to_texture(access_image("default:titlescreen_bad"));
	let logo = to_texture(access_image("default:logo"));

	// The menu
	let y_pos =
		|height: f32| (screen_height() / 2.222) + (button_size().y * height) + (12. * height);

	loop {
		clear_background(GRAY);
		render_texture_fullscreen(&titlescreen);
		draw_texture_ex(
			&logo,
			0.,
			0.,
			WHITE,
			DrawTextureParams {
				dest_size: Some(
					vec2(logo.width(), logo.height()) * (average_screen_size() / 222.222),
				),
				..Default::default()
			},
		);

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

fn render_texture_fullscreen(texture: &Texture2D) {
	let width = texture.width();
	let height = texture.height();

	let scale_x: f32;
	let scale_y: f32;

	if screen_width() < screen_height() * (width / height) {
		scale_y = screen_height();
		scale_x = scale_y * (width / height);
	} else {
		scale_x = screen_width();
		scale_y = scale_x * (height / width);
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
