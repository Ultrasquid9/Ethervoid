use macroquad::{prelude::*, ui::*};

use crate::gameplay::npc::messages::Message;

pub fn menu(message: &mut Message) {
	let height = screen_height() / 2.5;

	let pos = vec2(12., screen_height() - height);
	let size = vec2(screen_width() - 24., height - 12.);

	widgets::Window::new(hash!(), pos, size)
		.movable(false)
		.ui(&mut root_ui(), |ui| {
			ui.label(None, message.get_dialogue().get_text());

			if ui.button(vec2(12., 12.), "Next") {
				message.next();
			}
		});
}
