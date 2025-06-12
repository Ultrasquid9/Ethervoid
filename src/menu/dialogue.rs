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
		.ui(&mut root_ui(), |ui| ui.label(None, &label_dialogue));
}
