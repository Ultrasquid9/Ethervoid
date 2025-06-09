use macroquad::prelude::*;
use mlua::{FromLua, IntoLua, Lua, Number, Value};

use crate::utils::error::EvoidResult;

pub struct LuaDVec2(pub DVec2);

impl IntoLua for LuaDVec2 {
	fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
		let table = lua.create_table()?;
		let inner = self.0;

		table.set("x", inner.x)?;
		table.set("y", inner.y)?;

		Ok(Value::Table(table))
	}
}

impl FromLua for LuaDVec2 {
	fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
		let Some(table) = value.as_table() else {
			todo!("error handling")
		};

		Ok(Self(dvec2(
			table.get("x")?, 
			table.get("y")?,
		)))
	}
}

pub fn create_lua() -> Lua {
	fn create_lua_inner() -> EvoidResult<Lua> {
		let lua = Lua::new();
		let globals = lua.globals();

		globals.set(
			"angle_between",
			lua.create_function(|_, args: (LuaDVec2, LuaDVec2)| {
			Ok((args.1.0.y - args.0.0.y).atan2(args.1.0.x - args.0.0.x))
			})?
		)?;
		globals.set(
			"distance_between",
			lua.create_function(|_, args: (LuaDVec2, LuaDVec2)| {
				Ok(args.1.0.distance(args.0.0))
			})?
		)?;
		globals.set(
			"delta_time",
			lua.create_function(|_, ()| {
				Ok(get_frame_time())
			})?
		)?;
		globals.set(
			"round",
			lua.create_function(|_, num: Number| {
				Ok(num.round())
			})?
		)?;

		lua.sandbox(true)?;
		Ok(lua)
	}
	
	match create_lua_inner() {
		Ok(ok) => ok,
		Err(e) => {
			error!("{e}");
			panic!("{e}")
		}
	}
}
