use crate::{gameplay::npc::messages::Message, prelude::*};

use super::{Readable, gen_name, get_files};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum NpcMovement {
	Wander(f64),
	Still,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NpcType {
	pub name: String,
	pub sprite: String,
	pub movement: NpcMovement,
	pub messages: Box<[Message]>,
}

impl Readable for NpcType {}

/// Provides a HashMap containing all Npc data
pub fn get_npctypes() -> HashMap<String, NpcType> {
	let npcs: HashMap<String, NpcType> = get_files("npcs".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), NpcType::read(dir)))
		.filter_map(|(str, result)| match result {
			Err(e) => {
				warn!("Npc {str} failed to load: {e}");
				None
			}
			Ok(npctype) => {
				info!("Npc {str} loaded!");
				Some((str, npctype))
			}
		})
		.collect();

	npcs
}
