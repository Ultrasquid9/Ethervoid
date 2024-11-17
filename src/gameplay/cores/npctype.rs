use std::fs;
use ahash::HashMap;
use macroquad::prelude::rand;

use serde::{
	Deserialize, 
	Serialize
};

use crate::gameplay::npc::{
	Dialogue, 
	NPCMovement
};

use super::{
	gen_name, 
	get_files
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
	required_flags: Vec<String>,
	unlocked_flags: Vec<String>,
	probability: u8,
	text: Vec<Dialogue>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NPCType {
	pub name: String,
	pub sprite: String,
	pub movement: NPCMovement,
	pub messages: Vec<Message>
}

impl NPCType {
	pub fn read(dir: String) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}
}

impl Message {
	// TODO - Make
	pub fn should_read(&self) -> bool {
		if rand::gen_range(0, 255) > self.probability {
			return false 
		}

		return true;
	}

	// TODO - Make 
	pub fn read(&self) {
		for i in &self.text {
			i.read();
		}
	}
}

/// Provides a HashMap containing all NPC data
pub fn get_npctypes() -> HashMap<String, NPCType> {
	let mut npcs: HashMap<String, NPCType> = HashMap::default();

	for i in get_files(String::from("npcs")) {
		npcs.insert(
			gen_name(&i),
			NPCType::read(i)
		);
	}

	return npcs;
}
