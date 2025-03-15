use crate::State;
use macroquad::prelude::*;
use ui::FONT;

pub mod ui;

/// The main menu of the game
pub async fn menu() -> State {
	let mut to_return: Option<State> = None; // The state that will be returned to Main

	// The menu
	loop {
		clear_background(GRAY);

		yakui_macroquad::start();

		let mut column = yakui::widgets::List::column();
		column.main_axis_alignment = yakui::MainAxisAlignment::Center;
		column.cross_axis_alignment = yakui::CrossAxisAlignment::End;

		column.show(|| {
			if button("Play") {
				to_return = Some(State::Gameplay)
			}
			if button("Quit") {
				to_return = Some(State::Quit)
			}
		});

		yakui_macroquad::finish();
		yakui_macroquad::draw();

		if let Some(state) = to_return {
			return state;
		}

		next_frame().await
	}
}

fn button(str: &str) -> bool {
	let mut button = yakui::widgets::Button::styled(str.to_owned());
	button.style.text.font = FONT.clone();
	button.hover_style.text.font = FONT.clone();
	button.down_style.text.font = FONT.clone();

	button.show().clicked
}
