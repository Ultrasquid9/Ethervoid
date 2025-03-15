use super::World;
use parking_lot::RwLock;
use player::player_behavior;
use script::script_behavior;
use stecs::prelude::*;

use crate::{
	cores::script::Script,
	utils::{get_delta_time, resources::maps::access_map},
};

use std::{error::Error, thread};

use macroquad::{math::DVec2, prelude::rand};

pub mod player;
pub mod script;

#[derive(PartialEq, Clone)]
pub enum Behavior {
	Player(PlayerBehavior),
	Enemy(EnemyBehavior),
	Wander(WanderBehavior),
	None,
}

#[derive(PartialEq, Clone)]
pub struct PlayerBehavior {
	pub speed: f64,

	pub dash_cooldown: f64,
	pub is_dashing: bool,
}

pub struct EnemyBehavior {
	pub movement: Script,
	pub attacks: Box<[Script]>,

	pub attack_index: usize,
	pub attack_cooldown: f64,

	pub err: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(PartialEq, Clone)]
pub struct WanderBehavior {
	pub pos: DVec2,
	pub range: f64,

	pub cooldown: f64,
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

impl PartialEq for EnemyBehavior {
	fn eq(&self, other: &Self) -> bool {
		self.attacks.len() == other.attacks.len()
			&& self.attack_index == other.attack_index
			&& self.attack_cooldown == other.attack_cooldown
	}
}

impl Clone for EnemyBehavior {
	fn clone(&self) -> Self {
		Self {
			movement: self.movement.clone(),
			attacks: self.attacks.clone(),

			attack_index: self.attack_index,
			attack_cooldown: self.attack_cooldown,

			err: None,
		}
	}
}

pub fn handle_behavior(world: &mut World) {
	let obj_player = *world.player.obj.first().unwrap();

	let attacks = RwLock::new(&mut world.attacks);

	thread::scope(|scope| {
		for (obj, behavior, sprite) in query!(
			[world.player, world.enemies, world.npcs],
			(&mut obj, &mut behavior, &mut sprite)
		) {
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
				Behavior::Player(behavior) => {
					player_behavior(obj, behavior, &world.config, &world.current_map)
				}

				Behavior::Enemy(behavior) => {
					if behavior.err.is_some() {
						continue;
					}

					scope.spawn(|| {
						let result = script_behavior(
							if behavior.attack_cooldown > 0. {
								behavior.attack_cooldown -= get_delta_time();
								&mut behavior.movement
							} else {
								&mut behavior.attacks[behavior.attack_index]
							},
							obj,
							&obj_player,
							sprite,
							*attacks.write(),
							&world.current_map,
						);

						match result {
							Ok(i) if i => behavior.change_attack_index(),
							Err(e) => {
								println!("Script error: {e}");
								behavior.err = Some(e)
							}

							_ => (),
						}
					});
				}

				Behavior::Wander(behavior) => {
					if behavior.cooldown > 0. {
						behavior.cooldown -= get_delta_time();
						continue;
					} else if obj.pos.distance(obj.target) < 5. {
						obj.update(DVec2::new(
							rand::gen_range(
								behavior.pos.x - behavior.range,
								behavior.pos.x + behavior.range,
							),
							rand::gen_range(
								behavior.pos.y - behavior.range,
								behavior.pos.y + behavior.range,
							),
						));
						behavior.cooldown = 120.;
						continue;
					}
					let new_pos = obj.pos.move_towards(obj.target, 2. * get_delta_time());
					obj.try_move(new_pos, &world.current_map);

					if obj.pos != new_pos {
						obj.target = obj.pos
					}
				}

				_ => (),
			}
		}
	});

	for door in access_map(&world.current_map).doors.clone() {
		door.try_change_map(world);
	}
}
