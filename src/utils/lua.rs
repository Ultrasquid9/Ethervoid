use macroquad::prelude::*;
use mlua::{FromLua, IntoLua, Lua, Value};

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
