use crate::{
	State,
	menu::{dialogue, pause},
	utils::{get_delta_time, resources::config::access_config},
};

use super::npc::messages::Message;

#[derive(PartialEq)]
pub enum Paused {
	Dialogue(Option<Message>),
	Hitstop(f64),
	Pause,
	None,
}

impl Paused {
	pub fn is_paused(&self) -> bool {
		*self != Paused::None
	}

	pub fn pause(&mut self) -> Option<State> {
		match self {
			Self::Dialogue(message) => {
				if let Some(message) = message {
					dialogue::menu(message);

					if message.should_stop() {
						*self = Self::None;
					}
				} else {
					*self = Self::None;
				}
			}

			Self::Hitstop(hitstop) => {
				if *hitstop <= 0. {
					*self = Self::None;
				} else {
					*hitstop -= get_delta_time();
				}
			}

			Self::Pause => {
				if access_config().keymap.pause.is_pressed() {
					*self = Self::None;
				}

				return pause::menu();
			}

			Self::None => {
				if access_config().keymap.pause.is_pressed() {
					*self = Paused::Pause
				}
			}
		}

		None
	}
}
