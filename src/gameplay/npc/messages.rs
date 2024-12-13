use macroquad::prelude::*;

use serde::{
	Deserialize, 
	Serialize
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
	required_flags: Vec<String>,
	unlocked_flags: Vec<String>,
	probability: u8,
	text: Vec<Dialogue>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
	name: String,
	portrait: String,
	text: String
}

impl Message {
	// TODO - Make
	pub fn should_read(&self) -> bool {
		if rand::gen_range(0, 255) > self.probability {
			return false 
		}

		true
	}

	// TODO - Make 
	pub fn read(&self) {
		for i in &self.text {
			i.read();
		}
	}
}

impl Dialogue {
	/// Reading dialogue
	/// Note: HIGHLY WIP
	pub fn read(&self) { println!("{}", self.text) } 
}
