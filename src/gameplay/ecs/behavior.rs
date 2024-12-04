use super::World;
use player::player_behavior;
use script::script_behavior;
use stecs::prelude::*;

use crate::{
	utils::{
		resources::maps::access_map,
		get_delta_time
	},
	cores::script::Script
};

use macroquad::{
	math::Vec2, 
	prelude::rand
};

pub mod script;
pub mod player;

#[derive(PartialEq, Clone)]
pub enum Behavior {
	Player(PlayerBehavior),
	Enemy(EnemyBehavior),
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
pub struct EnemyBehavior {
	pub movement: Script,
	pub attacks: Vec<Script>,

	pub attack_index: usize,
	pub attack_cooldown: f32,
}

#[derive(PartialEq, Clone)]
pub struct WanderBehavior {
	pub pos: Vec2,
	pub range: f32,

	pub cooldown: f32
}

impl EnemyBehavior {
	fn change_attack_index(&mut self) {
		self.attack_cooldown = 40.;
		self.movement.scope.clear();

		self.attack_index = if self.attack_index >= self.attacks.len() - 1 {
			0
		} else {
			self.attack_index + 1
		};
	}
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = *world.player.obj.first().unwrap();

	for (obj, behavior) in query!([world.player, world.enemies, world.npcs], (&mut obj, &mut behavior)) {
		if obj.stunned > 0. { 
			obj.stunned -= get_delta_time();

			if let Behavior::Enemy(behavior) = behavior {
				if behavior.attack_cooldown <= 0. {
					behavior.change_attack_index();

					for script in &mut behavior.attacks {
						script.scope.clear();
					}
				}
			}

			continue;
		}

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
						&mut behavior.movement
					} else {
						&mut behavior.attacks[behavior.attack_index]
					}, 
					obj, 
					&obj_player, 
					&mut world.attacks,
					&world.current_map
				) { behavior.change_attack_index() };
			},

			Behavior::Wander(behavior) => {
				if behavior.cooldown > 0. {
					behavior.cooldown -= get_delta_time();
					continue;
				} else if obj.pos.distance(obj.target) < 5. {
					obj.update(Vec2::new(
						rand::gen_range(behavior.pos.x - behavior.range, behavior.pos.x + behavior.range),
						rand::gen_range(behavior.pos.y - behavior.range, behavior.pos.y + behavior.range)
					));
					behavior.cooldown = 120.;
					continue;
				}
				let new_pos = obj.pos.move_towards(obj.target, 2. * get_delta_time());
				obj.try_move(new_pos, &world.current_map);

				if obj.pos != new_pos {
					obj.target = obj.pos
				}
			},

			_ => ()
		}
	}

	for door in access_map(&world.current_map).doors {
		door.try_change_map(world);
	}
}
