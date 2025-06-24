use std::error::Error;

use mlua::Function;
use tracing::error;

use crate::{
	cores::script::Script,
	gameplay::ecs::{obj::Obj, sprite::Sprite},
	utils::{ImmutVec, error::EvoidResult, lua::LuaDVec2, resources::scripts::lua, smart_time},
};

pub struct Goals {
	pub scripts: ImmutVec<Script>,
	pub prev_script: String,
	pub index: Option<usize>,

	pub err: Option<Box<dyn Error + Send + Sync>>,
}

impl Goals {
	pub fn new(scripts: ImmutVec<Script>) -> Self {
		Self {
			scripts,
			prev_script: "none".to_owned(),

			index: None,
			err: None,
		}
	}

	pub fn from_scripts(scripts: &ImmutVec<String>) -> Self {
		Self::new(
			scripts
				.iter()
				.filter_map(|key| match Script::new(key) {
					Ok(ok) => Some(ok),
					Err(e) => {
						error!("Failed to eval script {key}: {e}");
						None
					}
				})
				.collect::<ImmutVec<Script>>(),
		)
	}

	/// Runs the entity's current goal
	pub fn run_goal(
		&mut self,
		obj_self: &mut Obj,
		obj_player: &Obj,
		sprite: &mut Sprite,
		current_map: &str,
	) {
		// Macro to execute a function and check if it returns an error
		macro_rules! maybe {
			($EvoidResult:expr) => {
				match $EvoidResult {
					Err(e) => {
						error!("{e}");
						self.err = Some(e);
						return;
					}
					Ok(ok) => ok,
				}
			};
		}

		if self.err.is_some() {
			return;
		}

		// Updates the current goal, and checks it it should be stopped
		if let Some(index) = self.index {
			maybe!(self.scripts[index].update(obj_self, obj_player, sprite, current_map));

			if maybe!(self.scripts[index].should_stop(obj_self, obj_player)) {
				sprite.set_default_anim();
				self.prev_script.clone_from(&self.scripts[index].name);
				self.index = None;
			}

			return;
		}

		// Checks each goal to see if they should be started, and selects the first valid one
		for index in 0..self.scripts.len() {
			if maybe!(self.scripts[index].should_start(obj_self, obj_player, &self.prev_script)) {
				maybe!(self.scripts[index].init(obj_self, obj_player));
				self.index = Some(index);
				return;
			}
		}
	}
}

impl PartialEq for Goals {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index && self.prev_script == other.prev_script
	}
}

impl Clone for Goals {
	fn clone(&self) -> Self {
		Self {
			scripts: self.scripts.clone(),
			prev_script: self.prev_script.clone(),
			index: self.index,
			err: None,
		}
	}
}

impl Script {
	fn init(&mut self, obj_self: &Obj, obj_player: &Obj) -> EvoidResult<()> {
		let fun: Function = match self.table()?.get("init") {
			Ok(fun) => fun,
			Err(e) => match e {
				mlua::Error::FromLuaConversionError { .. } => return Ok(()),
				other => return Err(other.into()),
			},
		};

		Ok(fun.call((
			self.table()?.clone(),
			LuaDVec2(obj_self.pos),
			LuaDVec2(obj_player.pos),
		))?)
	}

	fn should_start(
		&mut self,
		obj_self: &Obj,
		obj_player: &Obj,
		prev_goal: &str,
	) -> EvoidResult<bool> {
		let fun: Function = self.table()?.get("should_start")?;
		Ok(fun.call((
			self.table()?.clone(),
			LuaDVec2(obj_self.pos),
			LuaDVec2(obj_player.pos),
			prev_goal,
		))?)
	}

	fn should_stop(&mut self, obj_self: &Obj, obj_player: &Obj) -> EvoidResult<bool> {
		let fun: Function = self.table()?.get("should_stop")?;
		Ok(fun.call((
			self.table()?.clone(),
			LuaDVec2(obj_self.pos),
			LuaDVec2(obj_player.pos),
		))?)
	}

	fn update(
		&mut self,
		obj_self: &mut Obj,
		obj_player: &Obj,
		sprite: &mut Sprite,
		current_map: &str,
	) -> EvoidResult<()> {
		let lua_current_anim =
			lua().create_string(sprite.get_current_anim().unwrap_or_default())?;

		let fun: Function = self.table()?.get("update")?;
		let new_pos: LuaDVec2 = fun.call((
			self.table()?.clone(),
			LuaDVec2(obj_self.pos),
			LuaDVec2(obj_player.pos),
			lua_current_anim.clone(),
		))?;

		// Setting a new anim (if one was set)
		let current_anim = lua_current_anim.to_str()?.to_string();
		if !current_anim.is_empty()
			&& !matches!(sprite.get_current_anim(), Some(anim) if anim == current_anim)
		{
			sprite.set_new_anim(current_anim)?;
		}

		// Taking delta time into consideration
		let new_pos = ((*new_pos - obj_self.pos) * smart_time()) + obj_self.pos;

		obj_self.update(new_pos);
		obj_self.try_move(&new_pos, current_map);

		Ok(())
	}
}
