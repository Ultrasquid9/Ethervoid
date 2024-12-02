use macroquad::{math::Vec2, prelude::rand};
use player::player_behavior;
use script::script_behavior;
use stecs::prelude::*;

use crate::{cores::script::Script, utils::{get_delta_time, resources::maps::access_map}};

use super::World;

pub mod script;
pub mod player;

#[derive(PartialEq, Clone)]
pub enum Behavior<'a> {
	Player(PlayerBehavior),
	Enemy(EnemyBehavior<'a>),
	Wander(WanderBehavior),
	None
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

#[derive(PartialEq, Clone)]
pub struct WanderBehavior {
	pub pos: Vec2,
	pub range: f32,

	pub cooldown: f32
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = *world.player.obj.first().unwrap();

	for (obj, behavior) in query!([world.player, world.enemies, world.npcs], (&mut obj, &mut behavior)) {
		match behavior {
			Behavior::Player(behavior) => player_behavior(
				obj, 
				behavior,
				&world.config,
				&world.current_map
			),

			Behavior::Enemy(behavior) => {
				if script_behavior(
					if behavior.attack_cooldown > 0. {
						behavior.attack_cooldown -= get_delta_time();
						behavior.attacks[behavior.attack_index].current_target = obj_player.pos;

						&mut behavior.movement
					} else {
						&mut behavior.attacks[behavior.attack_index]
					}, 
					obj, 
					&obj_player, 
					&mut world.attacks,
					&world.current_map
				) {
					behavior.attack_index = if behavior.attack_index >= behavior.attacks.len() - 1 {
						0
					} else {
						behavior.attack_index + 1
					};

					behavior.attack_cooldown = 40.;
				};
			},

			Behavior::Wander(behavior) => {
				if behavior.cooldown > 0. {
					behavior.cooldown -= get_delta_time();
					continue;
				} else if obj.pos.distance(obj.target) < 5. {
					behavior.pos = Vec2::new(
						rand::gen_range(behavior.pos.x - behavior.range, behavior.pos.x + behavior.range),
						rand::gen_range(behavior.pos.y - behavior.range, behavior.pos.y + behavior.range)
					);
				}


			},

			_ => ()
		}
	}

	for door in access_map(&world.current_map).doors {
		door.try_change_map(world);
	}
}
