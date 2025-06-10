use std::error::Error;

use mlua::Function;
use tracing::error;

use crate::{
	cores::script::Script,
	gameplay::{
		combat::{Attack, AttackStructOf},
		ecs::{obj::Obj, sprite::Sprite},
	},
	utils::{error::EvoidResult, lua::LuaDVec2, resources::script_vals::lua, smart_time},
};

use stecs::{prelude::Archetype, storage::vec::VecFamily};

pub struct GoalBehavior {
	pub goals: Box<[Script]>,
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

impl Script {
	fn init(&mut self) -> EvoidResult<()> {
		let fun: Function = match self.table()?.get("init") {
			Ok(fun) => fun,
			Err(e) => match e {
				mlua::Error::FromLuaConversionError { .. } => return Ok(()),
				other => return Err(other.into()),
			},
		};

		Ok(fun.call(self.table()?.clone())?)
	}

	fn should_start(&mut self) -> EvoidResult<bool> {
		let fun: Function = self.table()?.get("should_start")?;
		Ok(fun.call(self.table()?.clone())?)
	}

	fn should_stop(&mut self, sprite: &mut Sprite) -> EvoidResult<bool> {
		let fun: Function = self.table()?.get("should_stop")?;
		let stop: bool = fun.call(self.table()?.clone())?;

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
		let lua_current_anim =
			lua().create_string(sprite.get_current_anim().unwrap_or_default())?;

		let fun: Function = self.table()?.get("update")?;
		let new_pos: LuaDVec2 = fun.call((
			self.table()?.clone(),
			lua_attacks.clone(),
			lua_current_anim.clone(),
		))?;

		// Getting attacks from the table
		lua_attacks.for_each(|_: String, atk: Attack| {
			attacks.insert(atk);
			Ok(())
		})?;

		// Setting a new anim (if one was set)
		let current_anim = lua_current_anim.to_str()?.to_string();
		if !current_anim.is_empty()
			&& !matches!(sprite.get_current_anim(), Some(anim) if anim == current_anim)
		{
			sprite.set_new_anim(current_anim)?;
		}

		// Taking delta time into consideration
		let new_pos = ((*new_pos - obj.pos) * smart_time()) + obj.pos;

		obj.update(new_pos);
		obj.try_move(&new_pos, current_map);

		Ok(())
	}
}

fn update_lua_constants(obj_self: &Obj, obj_player: &Obj, prev_goal: String) -> EvoidResult<()> {
	// Cloning them here to avoid lifetime issues
	let pos_player = obj_player.pos;
	let pos_self = obj_self.pos;

	let lua = lua();
	let globals = lua.globals();

	let position = lua.create_table()?;
	position.set(
		"player",
		lua.create_function(move |_, ()| Ok(LuaDVec2(pos_player)))?,
	)?;
	position.set(
		"self",
		lua.create_function(move |_, ()| Ok(LuaDVec2(pos_self)))?,
	)?;

	let goals = lua.create_table()?;
	goals.set(
		"previous",
		lua.create_function(move |_, ()| Ok(prev_goal.clone()))?,
	)?;

	// Adding the tables to global
	globals.set("position", position)?;
	globals.set("goals", goals)?;

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

	maybe!(update_lua_constants(
		obj_self,
		obj_player,
		behavior.prev_goal.clone()
	));

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
		match behavior.goals[index].should_start() {
			Err(e) => {
				error!("{e}");
				behavior.err = Some(e);
			}
			Ok(true) => {
				behavior.index = Some(index);
				maybe!(behavior.goals[index].init());
				return;
			}
			_ => (),
		}
	}
}
