use parking_lot::RwLock;
use player::player_behavior;
use stecs::prelude::*;

use crate::{
	cores::goal::Goal,
	gameplay::Gameplay,
	utils::{get_delta_time, resources::maps::access_map},
};

use std::{error::Error, thread};

use macroquad::{math::DVec2, prelude::rand};

pub mod goal;
pub mod player;

#[derive(PartialEq, Clone)]
pub enum Behavior {
	Player(PlayerBehavior),
	Goal(GoalBehavior),
	Wander(WanderBehavior),
	None,
}

#[derive(PartialEq, Clone)]
pub struct PlayerBehavior {
	pub speed: f64,

	pub dash_cooldown: f64,
	pub is_dashing: bool,
}

pub struct GoalBehavior {
	pub goals: Box<[Goal]>,
	pub index: Option<usize>,

	pub err: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(PartialEq, Clone)]
pub struct WanderBehavior {
	pub pos: DVec2,
	pub range: f64,

	pub cooldown: f64,
}

impl PartialEq for GoalBehavior {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl Clone for GoalBehavior {
	fn clone(&self) -> Self {
		Self {
			goals: self.goals.clone(),
			index: self.index,
			err: None,
		}
	}
}

pub fn handle_behavior(gameplay: &mut Gameplay) {
	let obj_player = *gameplay.world.player.obj.first().unwrap();
	let attacks = RwLock::new(&mut gameplay.world.attacks);

	thread::scope(|_scope| {
		for (obj, behavior, sprite) in query!(
			[
				gameplay.world.player,
				gameplay.world.enemies,
				gameplay.world.npcs
			],
			(&mut obj, &mut behavior, &mut sprite)
		) {
			if obj.stunned > 0. {
				obj.stunned -= get_delta_time();

				if let Behavior::Goal(behavior) = behavior {
					behavior.index = None
				}

				continue;
			}

			match behavior {
				Behavior::Player(behavior) => {
					player_behavior(obj, behavior, &gameplay.config, &gameplay.current_map)
				}

				Behavior::Goal(behavior) => {
					// Probably not the most performant way to do it
					for script in behavior.goals.iter_mut() {
						script.update_constants(obj, &obj_player);
					}

					// Updates the current goal, and checks it it should be stopped
					if let Some(index) = behavior.index {
						let result = behavior.goals[index].update(
							obj,
							sprite,
							*attacks.write(),
							&gameplay.current_map,
						);
						if let Err(e) = result {
							behavior.err = Some(e);
							continue;
						}

						let result = behavior.goals[index].should_stop(sprite);
						if let Err(e) = result {
							behavior.err = Some(e);
							continue;
						}

						if result.unwrap() {
							behavior.index = None
						}
						continue;
					}

					// Checks each goal to see if they should be started, and selects the first valid one
					for index in 0..behavior.goals.len() {
						let result = behavior.goals[index].should_start();
						if let Err(e) = result {
							behavior.err = Some(e);
							continue;
						}

						if result.unwrap() {
							behavior.index = Some(index)
						} else {
							continue;
						}

						let result = behavior.goals[index].init();
						if let Err(e) = result {
							behavior.err = Some(e);
						}
						break;
					}
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
					obj.try_move(new_pos, &gameplay.current_map);

					if obj.pos != new_pos {
						obj.target = obj.pos
					}
				}

				_ => (),
			}
		}
	});

	for door in access_map(&gameplay.current_map).doors.clone() {
		door.try_change_map(gameplay);
	}
}
