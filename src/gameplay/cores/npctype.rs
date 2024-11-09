use std::fs;

use ahash::HashMap;
use serde::{Deserialize, Serialize};

use crate::gameplay::npc::NPCMovement;

use super::{gen_name, get_files};

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
	required_flag: Option<String>,
	probability: u8,
	// Character portrait should go here
	text: String,
	unlocked_flag: Option<String>
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
