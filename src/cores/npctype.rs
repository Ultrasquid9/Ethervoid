use std::fs;
use ahash::HashMap;
use rayon::prelude::*;

use crate::gameplay::npc::messages::Message;

use super::{
	gen_name, 
	get_files
};

use serde::{
	Deserialize, 
	Serialize
};

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
	pub fn read(dir: &str) -> Self {
		ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap()
	}
}

/// Provides a HashMap containing all Npc data
pub fn get_npctypes() -> HashMap<String, NpcType> {
	let npcs: HashMap<String, NpcType> = get_files("npcs".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), NpcType::read(dir)))
		.collect();

	npcs
}
