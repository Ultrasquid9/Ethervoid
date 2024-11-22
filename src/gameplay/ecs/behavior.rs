use player::player_movement;
use script::script_movement;
use stecs::prelude::*;

use crate::cores::script::Script;

use super::World;

pub mod script;
pub mod player;

#[derive(PartialEq)]
pub enum Behavior<'a> {
	Player,
	Script(Script<'a>) // TODO: Embed Rhai script/state/engine into this variant
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = world.player.obj.first().unwrap().clone();

	for (obj, behavior) in query!([world.player, world.enemies], (&mut obj, &mut behavior)) {
		match behavior {
			Behavior::Player => player_movement(
				obj, 
				&mut world.player.config.first().unwrap()
			),
			Behavior::Script(script) => script_movement(
				script, 
				obj, 
				&obj_player,
				&mut world.attacks
			)
		}
	}
}
