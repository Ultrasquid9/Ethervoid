use player::player_movement;
use stecs::prelude::*;

use crate::cores::script::Script;

use super::World;

pub mod script;
pub mod player;

pub enum Behavior<'a> {
	Player,
	Script(Script<'a>) // TODO: Embed Rhai script/state/engine into this variant
}

pub fn handle_behavior(world: &mut World) {
	for (obj, behavior) in query!([world.player, world.enemies], (&mut obj, &mut behavior)) {
		match behavior {
			Behavior::Player => player_movement(obj, &mut world.player.config.first().unwrap()),
			_ => todo!()
		}
	}
}
