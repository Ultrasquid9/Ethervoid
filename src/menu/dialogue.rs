use macroquad::{prelude::*, ui::*};

use crate::{gameplay::npc::messages::Message, utils::resources::langs::access_lang};

pub fn menu(message: &mut Message) {
	let height = screen_height() / 2.5;

	let pos = vec2(12., screen_height() - height);
	let size = vec2(screen_width() - 24., height - 12.);

	let label_next = access_lang("menu_dialogue_button_next");
	let label_dialogue = message.get_dialogue().get_text();

	widgets::Window::new(hash!(), pos, size)
		.movable(false)
		.ui(&mut root_ui(), |ui| {
			ui.label(None, &label_dialogue);

			if ui.button(vec2(12., 12.), label_next) {
				message.next();
			}
		});
}
