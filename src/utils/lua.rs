use std::ops::Deref;

use macroquad::prelude::*;
use mlua::{FromLua, IntoLua, Lua, Value};
use tracing::error;

mod instance;

pub struct LuaDVec2(pub DVec2);

impl IntoLua for LuaDVec2 {
	fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
		let table = lua.create_table()?;

		table.set("x", self.x)?;
		table.set("y", self.y)?;

		Ok(Value::Table(table))
	}
}

impl FromLua for LuaDVec2 {
	fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
		match value {
			Value::Table(table) => Ok(dvec2(table.get("x")?, table.get("y")?).into()),
			other => Err(mlua::Error::FromLuaConversionError {
				from: other.type_name(),
				to: "DVec2".into(),
				message: None,
			}),
		}
	}
}

impl Deref for LuaDVec2 {
	type Target = DVec2;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<DVec2> for LuaDVec2 {
	fn from(value: DVec2) -> Self {
		Self(value)
	}
}

impl From<Vec2> for LuaDVec2 {
	fn from(value: Vec2) -> Self {
		Self(dvec2(value.x as f64, value.y as f64))
	}
}

pub fn create_lua() -> Lua {
	match instance::try_create_lua() {
		Ok(ok) => ok,
		Err(e) => {
			error!("{e}");
			panic!("{e}")
		}
	}
}
