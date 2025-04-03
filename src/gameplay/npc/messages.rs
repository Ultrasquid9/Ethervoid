use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{gameplay::draw::process::to_texture, utils::resources::textures::access_image};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
	required_flags: Vec<String>,
	unlocked_flags: Vec<String>,
	probability: u8,
	text: Vec<Dialogue>,

	#[serde(skip)]
	index: usize,
	#[serde(skip)]
	should_stop: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
	name: String,
	portrait: String,
	text: String,
}

impl Message {
	/// Checks if the message is able to be read
	pub fn should_read(&self) -> bool {
		// TODO - Consider flags
		rand::gen_range(0, 255) <= self.probability
	}

	/// Gets the message's current dialogue based on its index
	pub fn get_dialogue(&self) -> &Dialogue {
		self.text
			.get(self.index)
			.expect("Index should always point to valid data")
	}

	/// Increases the index if possible
	pub fn next(&mut self) {
		if self.text.get(self.index + 1).is_some() {
			self.index += 1;
		} else {
			self.should_stop = true;
		}
	}

	/// Checks if the message should continue
	pub fn should_stop(&self) -> bool {
		self.should_stop
	}
}

#[allow(unused)] // TODO: Make them no longer unused
impl Dialogue {
	/// Gets the name of the character as a &str
	pub fn get_name(&self) -> &str {
		&self.name
	}

	/// Gets the text inside as a &str
	pub fn get_text(&self) -> &str {
		&self.text
	}

	/// Gets the portrait inside as a Texture2D
	pub fn get_portrait(&self) -> Texture2D {
		to_texture(access_image(&self.portrait))
	}
}
