use std::error::Error;

use tracing::error;
use macroquad::math::DVec2;
use rhai::{CallFnOptions, Dynamic};

use crate::{
	cores::goal::Goal,
	gameplay::{
		combat::AttackStructOf,
		ecs::{obj::Obj, sprite::Sprite},
	},
	utils::{error::EvoidResult, get_delta_time},
};

use stecs::{prelude::*, storage::vec::VecFamily};

pub struct GoalBehavior {
	pub goals: Box<[Goal]>,
	pub prev_goal: String,
	pub index: Option<usize>,

	pub err: Option<Box<dyn Error + Send + Sync>>,
}

impl PartialEq for GoalBehavior {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index && self.prev_goal == other.prev_goal
	}
}

impl Clone for GoalBehavior {
	fn clone(&self) -> Self {
		Self {
			goals: self.goals.clone(),
			prev_goal: self.prev_goal.clone(),
			index: self.index,
			err: None,
		}
	}
}

impl Goal {
	fn update_constants(&mut self, obj_self: &Obj, obj_player: &Obj, prev_goal: String) {
		// The names of the constants
		const PREV_GOAL: &str = "prev_goal";
		const POS_SELF: &str = "pos_self";
		const POS_PLAYER: &str = "pos_player";

		// Removing the constants if they exist, to prevent the scope from growing exponentially
		_ = self.scope.remove::<Dynamic>(PREV_GOAL);
		_ = self.scope.remove::<Dynamic>(POS_SELF);
		_ = self.scope.remove::<Dynamic>(POS_PLAYER);

		// Re-adding the constants with updated values
		self.scope
			.push_constant(PREV_GOAL, prev_goal)
			.push_constant(POS_SELF, obj_self.pos)
			.push_constant(POS_PLAYER, obj_player.pos);
	}

	fn should_start(&mut self) -> EvoidResult<bool> {
		let res = self
			.engine
			.call_fn::<bool>(&mut self.scope, &self.script, "should_start", ())?;

		Ok(res)
	}

	fn should_stop(&mut self, sprite: &mut Sprite) -> EvoidResult<bool> {
		let res = self
			.engine
			.call_fn::<bool>(&mut self.scope, &self.script, "should_stop", ())?;

		if res {
			self.scope.clear();
			sprite.set_default_anim();
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn init(&mut self) -> EvoidResult<()> {
		self.engine.call_fn_with_options::<()>(
			CallFnOptions::new().eval_ast(false).rewind_scope(false),
			&mut self.scope,
			&self.script,
			"init",
			(),
		)?;

		Ok(())
	}

	fn update(
		&mut self,
		obj: &mut Obj,
		sprite: &mut Sprite,
		attacks: &mut AttackStructOf<VecFamily>,
		current_map: &str,
	) -> EvoidResult<()> {
		// Values available in the scope
		self.scope
			.push("attacks", Vec::<Dynamic>::new())
			.push("current_anim", String::new());

		// Executing the script
		let new_pos = self
			.engine
			.call_fn::<DVec2>(&mut self.scope, &self.script, "update", ())?;

		// Getting attacks out of the scope
		let new_attacks = self
			.scope
			.remove::<Vec<Dynamic>>("attacks")
			.expect("Attacks not found");
		for attack in new_attacks {
			attacks.insert(attack.clone_cast());
		}

		// Getting the new animation from the scope
		let new_anim = self.scope.remove::<String>("current_anim");
		if new_anim.is_some() && !new_anim.as_ref().unwrap().is_empty() {
			sprite.set_new_anim(new_anim.unwrap())?;
		}

		// Taking delta time into consideration
		let new_pos = ((new_pos - obj.pos) * get_delta_time()) + obj.pos;

		obj.update(new_pos);
		obj.try_move(&new_pos, current_map);

		Ok(())
	}
}

pub fn goal_behavior(
	behavior: &mut GoalBehavior,
	obj_self: &mut Obj,
	obj_player: &Obj,
	sprite: &mut Sprite,
	attacks: &mut AttackStructOf<VecFamily>,
	current_map: &str,
) {
	// Macro to execute a function and check if it returns an error
	macro_rules! maybe {
		($EvoidResult:expr) => {
			match $EvoidResult {
				Err(e) => {
					error!("{e}");
					behavior.err = Some(e);
					return;
				}
				Ok(ok) => ok,
			}
		};
	}

	if behavior.err.is_some() {
		return;
	}

	// Probably not the most performant way to do it
	behavior.goals.iter_mut().for_each(|script| {
		script.update_constants(obj_self, obj_player, behavior.prev_goal.clone());
	});

	// Updates the current goal, and checks it it should be stopped
	if let Some(index) = behavior.index {
		maybe!(behavior.goals[index].update(obj_self, sprite, attacks, current_map));
		let should_stop = maybe!(behavior.goals[index].should_stop(sprite));

		if should_stop {
			behavior.prev_goal.clone_from(&behavior.goals[index].name);
			behavior.index = None;
		}
		return;
	}

	// Checks each goal to see if they should be started, and selects the first valid one
	for index in 0..behavior.goals.len() {
		let result = behavior.goals[index].should_start();
		if let Err(e) = result {
			error!("{e}");
			behavior.err = Some(e);
			continue;
		}

		if let Ok(true) = result {
			behavior.index = Some(index);
		} else {
			continue;
		}

		maybe!(behavior.goals[index].init());
		return;
	}
}
