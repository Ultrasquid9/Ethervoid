use goal::{GoalBehavior, goal_behavior};
use mlua::Table;
use player::{PlayerBehavior, player_behavior};
use stecs::prelude::*;
use wander::{WanderBehavior, wander_behavior};

use crate::{
	gameplay::{Gameplay, combat::Attack},
	utils::{
		error::EvoidResult,
		resources::{config::access_config, maps::access_map, scripts::lua},
		smart_time,
	},
};

use std::thread;

pub mod goal;
pub mod player;
pub mod wander;

#[derive(PartialEq, Clone)]
pub enum Behavior {
	Player(PlayerBehavior),
	Goal(GoalBehavior),
	Wander(WanderBehavior),
	None,
}

pub fn handle_behavior(gameplay: &mut Gameplay) {
	let obj_player = *gameplay
		.world
		.player
		.obj
		.first()
		.expect("Player should exist");

	thread::scope(|scope| {
		for (obj, behavior, sprite) in query!(
			[
				gameplay.world.player,
				gameplay.world.enemies,
				gameplay.world.npcs
			],
			(&mut obj, &mut behavior, &mut sprite)
		) {
			if obj.stunned > 0. {
				obj.stunned -= smart_time();

				if let Behavior::Goal(behavior) = behavior {
					behavior.index = None;
				}

				continue;
			}

			match behavior {
				Behavior::Player(behavior) => {
					player_behavior(obj, &mut *behavior, &access_config(), &gameplay.current_map);
				}

				Behavior::Goal(behavior) => {
					scope.spawn(|| {
						goal_behavior(
							&mut *behavior,
							obj,
							&obj_player,
							sprite,
							&gameplay.current_map,
						);
					});
				}

				Behavior::Wander(behavior) => {
					scope.spawn(|| wander_behavior(&mut *behavior, obj, &gameplay.current_map));
				}

				Behavior::None => (),
			}
		}
	});

	if let Err(e) = retrieve_lua_attacks(gameplay) {
		tracing::error!("{e}");
	}

	for door in &access_map(&gameplay.current_map.clone()).doors {
		door.try_change_map(gameplay);
	}
}

fn retrieve_lua_attacks(gameplay: &mut Gameplay) -> EvoidResult<()> {
	let attacks = lua()
		.globals()
		.get::<Table>("attack")?
		.get::<Table>("_attacks")?;

	for attack in attacks.pairs::<mlua::Value, Attack>() {
		let (_, attack) = attack?;
		gameplay.world.attacks.insert(attack);
	}

	attacks.clear()?;
	Ok(())
}
