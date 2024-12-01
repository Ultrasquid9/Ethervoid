use player::player_behavior;
use script::script_behavior;
use stecs::prelude::*;

use crate::cores::script::Script;

use super::World;

pub mod script;
pub mod player;

#[derive(PartialEq, Clone)]
pub enum Behavior<'a> {
	Player,
	Script(Script<'a>) // TODO: Embed Rhai script/state/engine into this variant
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = *world.player.obj.first().unwrap();

	for (obj, behavior) in query!([world.player, world.enemies], (&mut obj, &mut behavior)) {
		match behavior {
			Behavior::Player => player_behavior(
				obj, 
				&world.config
			),
			Behavior::Script(script) => script_behavior(
				script, 
				obj, 
				&obj_player,
				&mut world.attacks
			)
		}
	}
}
