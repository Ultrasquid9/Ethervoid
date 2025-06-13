use crate::{gameplay::npc::messages::Message, prelude::*, utils::ImmutVec};

use super::{gen_name, get_files, read_from_path};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum NpcMovement {
	Wander(f64),
	Still,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NpcType {
	pub sprite: String,
	pub movement: NpcMovement,
	pub messages: ImmutVec<Message>,
}

/// Provides a `HashMap` containing all Npc data
pub fn get_npctypes() -> HashMap<String, NpcType> {
	let npcs: HashMap<String, NpcType> = get_files("npcs")
		.iter()
		.map(|dir| (gen_name(dir), read_from_path(dir)))
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
