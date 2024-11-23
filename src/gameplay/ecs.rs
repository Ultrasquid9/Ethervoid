use stecs::prelude::StructOf;

use super::{combat::Attack, enemy::Enemy, npc::Npc, player::Player};

pub mod behavior;
pub mod health;
pub mod obj;
pub mod sprite;

pub struct World<'a> {
	pub player: StructOf<Vec<Player<'a>>>,
	pub enemies: StructOf<Vec<Enemy<'a>>>,
	pub npcs: StructOf<Vec<Npc<'a>>>,
	pub attacks: StructOf<Vec<Attack>>
}
