use std::fs;
use ahash::HashMap;
use macroquad::prelude::rand;

use serde::{
	Deserialize, 
	Serialize
};

use crate::gameplay::npc::Dialogue;

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
pub enum NpcMovement {
	Wander(f32),
	Still
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NpcType {
	pub name: String,
	pub sprite: String,
	pub movement: NpcMovement,
	pub messages: Vec<Message>
}

impl NpcType {
	pub fn read(dir: String) -> Self {
		ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap()
	}
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

/// Provides a HashMap containing all Npc data
pub fn get_npctypes() -> HashMap<String, NpcType> {
	let mut npcs: HashMap<String, NpcType> = HashMap::default();

	for i in get_files(String::from("npcs")) {
		npcs.insert(
			gen_name(&i),
			NpcType::read(i)
		);
	}

	npcs
}
