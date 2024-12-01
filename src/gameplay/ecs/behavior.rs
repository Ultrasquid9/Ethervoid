use player::player_behavior;
use script::script_behavior;
use stecs::prelude::*;

use crate::cores::script::Script;

use super::World;

pub mod script;
pub mod player;

#[derive(PartialEq, Clone)]
pub enum Behavior<'a> {
	Player(PlayerBehavior),
	Enemy(EnemyBehavior<'a>),
	Script(Script<'a>)
}

#[derive(PartialEq, Clone)]
pub struct PlayerBehavior {
	pub speed: f32,

	pub dash_cooldown: f32,
	pub is_dashing: bool
}

#[derive(PartialEq, Clone)]
pub struct EnemyBehavior<'a> {
	pub movement: Script<'a>,
	pub attacks: Vec<Script<'a>>,

	pub attack_index: usize,
	pub attack_cooldown: f32,
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = *world.player.obj.first().unwrap();

	for (obj, behavior) in query!([world.player, world.enemies], (&mut obj, &mut behavior)) {
		match behavior {
			Behavior::Player(behavior) => player_behavior(
				obj, 
				behavior,
				&world.config
			),
			Behavior::Script(script) => script_behavior(
				script, 
				obj, 
				&obj_player,
				&mut world.attacks
			),

			_ => ()
		}
	}
}
