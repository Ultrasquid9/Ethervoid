use ahash::HashMap;

use stecs::{
	prelude::*, 
	storage::vec::VecFamily
};

use super::{
	combat::Attack, 
	cores::map::Map, 
	enemy::Enemy, 
	npc::NPC, 
	player::Player
};

// This module manages the ECS.
// It is in a folder because I expect there will be submodules for it soon. 
//
// TODO - Make entities actually use components

#[derive(SplitFields)]
pub struct PlayerArch{
	pub io: Player	
}

#[derive(SplitFields)]
pub struct EnemyArch{
	pub io: Enemy	
}

#[derive(SplitFields)]
pub struct NPCArch{
	pub io: NPC
}

#[derive(SplitFields)]
pub struct AttackArch{
	pub io: Attack
}

// These types allow for references to parts of the world to be passed around easily.
// Dead code is allowed since I may need them in the future. 
#[allow(dead_code)]
pub type Enemies = EnemyArchStructOf<VecFamily>;
#[allow(dead_code)]
pub type NPCs = NPCArchStructOf<VecFamily>;
#[allow(dead_code)]
pub type Attacks = AttackArchStructOf<VecFamily>;

/// Contains the contents of the game
/// So far, this includes:
/// - Enemies
/// - NPCs
/// - Attacks
/// 
/// More will likely be added or migrated to this struct in the future
pub struct World {
	pub player: StructOf<Vec<PlayerArch>>,
	pub enemies: StructOf<Vec<EnemyArch>>,
	pub npcs: StructOf<Vec<NPCArch>>,
	pub attacks: StructOf<Vec<AttackArch>>,

	// Resources 
	pub hitstop: f32,

	// Maps
	pub maps: HashMap<String, Map>,
	pub current_map: String
}

impl World {
	pub fn get_current_map(&self) -> Map {
		return self.maps.get(&self.current_map).unwrap().clone();
	}
}