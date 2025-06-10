use std::ops::Deref;

use macroquad::prelude::*;
use mlua::{FromLua, IntoLua, Lua, Number, Table, Value};
use tracing::error;

use crate::{
	gameplay::{
		combat::{Attack, Owner},
		ecs::obj::Obj,
	},
	utils::{delta_time, error::EvoidResult},
};

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

macro_rules! lua_fn {
	($lua:ident, | $($arg:ident : $typ:ty),* $(,)? | $content:expr ) => {
		$lua.create_function(
			|_, ( $($arg,)* ) : ( $($typ,)* )| Ok($content)
		)?
	};
	($lua:ident, $content:expr ) => {
		$lua.create_function(
			|_, ()| Ok($content())
		)?
	};
}

pub fn create_lua() -> Lua {
	fn create_lua_inner() -> EvoidResult<Lua> {
		let lua = Lua::new();
		let globals = lua.globals();

		globals.set("delta_time", lua_fn!(lua, delta_time))?;
		globals.set("round", lua_fn!(lua, |num: Number| num.round()))?;
		globals.set(
			"angle_between",
			lua_fn!(lua, |current: LuaDVec2, target: LuaDVec2| {
				(target.y - current.y).atan2(target.x - current.x)
			}),
		)?;
		globals.set(
			"distance_between",
			lua_fn!(lua, |current: LuaDVec2, target: LuaDVec2| {
				current.distance(*target)
			}),
		)?;
		globals.set(
			"move_towards",
			lua_fn!(lua, |current: LuaDVec2, target: LuaDVec2, amount: _| {
				LuaDVec2(current.move_towards(*target, amount))
			}),
		)?;
		globals.set("attack", lua_attack_fns(&lua)?)?;

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

#[rustfmt::skip]
fn lua_attack_fns(lua: &Lua) -> EvoidResult<Table> {
	let attacks = lua.create_table()?;

	attacks.set(
		"physical",
		lua_fn!(lua, |damage: _, size: _, pos: LuaDVec2, target: LuaDVec2, key: String| {
			Attack::new_physical(Obj::new(*pos, *target, size), damage, Owner::Enemy, &key)
		}),
	)?;
	attacks.set(
		"burst",
		lua_fn!(lua, |damage: _, size: _, pos: LuaDVec2, key: String| {
			Attack::new_burst(Obj::new(*pos, *pos, size), damage, Owner::Enemy, &key)
		}),
	)?;
	attacks.set(
		"projectile",
		lua_fn!(lua, |damage: _, size: _, pos: LuaDVec2, target: LuaDVec2, key: String| {
			Attack::new_projectile(Obj::new(*pos, *target, size), damage, Owner::Enemy, &key)
		}),
	)?;
	attacks.set(
		"hitscan",
		lua_fn!(lua, |damage: _, size: _, pos: LuaDVec2, target: LuaDVec2, key: String| {
			Attack::new_hitscan(Obj::new(*pos, *target, size), damage, Owner::Enemy, &key)
		}),
	)?;

	Ok(attacks)
}
