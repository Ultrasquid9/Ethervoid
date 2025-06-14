use macroquad::{prelude::*, ui::*};

use crate::{
	gameplay::npc::messages::Message,
	menu::{button, button_size},
	utils::resources::langs::access_lang,
};

pub fn menu(message: &mut Message) {
	let height = screen_height() / 2.5;
	let pos_height = screen_height() - height;

	let pos = vec2(12., pos_height);
	let size = vec2(screen_width() - 24., height - 12.);

	let label_next = access_lang("menu_dialogue_button_next");
	let label_name = message.get_dialogue().get_name();
	let label_dialogue = message.get_dialogue().get_text();

	if button(&label_next, pos_height - button_size().y - 12.) {
		message.next();
	}

	widgets::Window::new(hash!(), pos, size)
		.label(&label_name)
		.titlebar(true)
		.movable(false)
		.ui(&mut root_ui(), |ui| {
			for line in calc_text_lines(ui, size, &label_dialogue) {
				ui.label(None, &line);
			}
		});
}

fn calc_text_lines(ui: &mut Ui, size: Vec2, text: &str) -> Vec<String> {
	let mut lines = vec![];
	let mut current_line = String::with_capacity(text.len());

	for word in text.split_whitespace() {
		let new_line = current_line.clone() + word;

		if ui.calc_size(&new_line).x > size.x {
			lines.push(current_line.clone());
			current_line.clear();
			current_line.push_str(word);
			current_line.push(' ');
		} else {
			current_line.clone_from(&new_line);
			current_line.push(' ');
		}
	}

	lines.push(current_line);
	lines
}
