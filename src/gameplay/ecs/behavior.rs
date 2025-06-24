use mlua::Table;
use stecs::prelude::*;

use crate::{
	gameplay::{Gameplay, combat::Attack},
	utils::{
		error::EvoidResult,
		resources::{maps::access_map, scripts::lua},
		smart_time,
	},
};

use super::obj::Obj;

pub mod goal;
pub mod player;

pub fn handle_behavior(gameplay: &mut Gameplay) {
	let mut obj_player = Obj::default();

	for (obj, controller) in query!(gameplay.world.player, (&mut obj, &mut controller)) {
		obj_player = *obj;
		controller.control(obj, &gameplay.current_map);
	}

	rayon::scope(|scope| {
		for (obj, goals, sprite) in query!(
			[gameplay.world.enemies, gameplay.world.npcs],
			(&mut obj, &mut goals, &mut sprite)
		) {
			if obj.stunned > 0. {
				obj.stunned -= smart_time();
				goals.index = None;
				continue;
			}

			scope.spawn(|_| {
				goals.run_goal(obj, &obj_player, sprite, &gameplay.current_map);
			});
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
