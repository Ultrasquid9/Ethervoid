use std::error::Error;

use mlua::Function;
use tracing::error;

use crate::{
	cores::goal::Goal,
	gameplay::{
		combat::AttackStructOf,
		ecs::{obj::Obj, sprite::Sprite},
	},
	utils::{error::EvoidResult, get_delta_time, lua::LuaDVec2, resources::goals::lua},
};

use stecs::{storage::vec::VecFamily};

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
	fn init(&mut self) -> EvoidResult<()> {
		let fun: Function = self.table.get("init")?;
		Ok(fun.call(self.table.clone())?)
	}

	fn should_start(&mut self) -> EvoidResult<bool> {
		let fun: Function = self.table.get("should_start")?;
		Ok(fun.call(self.table.clone())?)
	}

	fn should_stop(&mut self, sprite: &mut Sprite) -> EvoidResult<bool> {
		let fun: Function = self.table.get("should_stop")?;
		let stop: bool = fun.call(self.table.clone())?;

		if stop {
			sprite.set_default_anim();
		}
		Ok(stop)
	}

	fn update(
		&mut self,
		obj: &mut Obj,
		sprite: &mut Sprite,
		attacks: &mut AttackStructOf<VecFamily>,
		current_map: &str,
	) -> EvoidResult<()> {
		let lua_attacks = lua().create_table()?;
		let lua_current_anim = lua().create_string(sprite.get_current_anim().unwrap_or_default())?;

		let fun: Function = self.table.get("update")?;
		let new_pos: LuaDVec2 = fun.call((
			self.table.clone(),
			lua_attacks.clone(),
			lua_current_anim.clone(),
		))?;

		println!("{}", lua_attacks.len()?);
		
		// Taking delta time into consideration
		let new_pos = ((new_pos.0 - obj.pos) * get_delta_time()) + obj.pos;

		obj.update(new_pos);
		obj.try_move(&new_pos, current_map);

		Ok(())
	}
}

/* impl Goal {

	fn update2(
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
} */

fn update_lua_constants(obj_self: &Obj, obj_player: &Obj, prev_goal: String) -> EvoidResult<()> {
	// The names of the constants
	const PREV_GOAL: &str = "prev_goal";
	const POS_SELF: &str = "pos_self";
	const POS_PLAYER: &str = "pos_player";

	let globals = lua().globals();

	globals.set(PREV_GOAL, prev_goal)?;
	globals.set(POS_SELF, LuaDVec2(obj_self.pos))?;
	globals.set(POS_PLAYER, LuaDVec2(obj_player.pos))?;

	Ok(())
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

	maybe!(update_lua_constants(obj_self, obj_player, behavior.prev_goal.clone()));

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
			maybe!(behavior.goals[index].init());
			return;
		}
	}
}
