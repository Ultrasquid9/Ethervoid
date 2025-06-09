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
		let inner = self.0;

		table.set("x", inner.x)?;
		table.set("y", inner.y)?;

		Ok(Value::Table(table))
	}
}

impl FromLua for LuaDVec2 {
	fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
		let Some(table) = value.as_table() else {
			return Err(mlua::Error::FromLuaConversionError {
				from: value.type_name(),
				to: "DVec2".into(),
				message: None,
			});
		};

		Ok(Self(dvec2(table.get("x")?, table.get("y")?)))
	}
}

pub fn create_lua() -> Lua {
	fn create_lua_inner() -> EvoidResult<Lua> {
		let lua = Lua::new();
		let globals = lua.globals();

		globals.set("delta_time", lua.create_function(|_, ()| Ok(delta_time()))?)?;
		globals.set(
			"round",
			lua.create_function(|_, num: Number| Ok(num.round()))?,
		)?;
		globals.set(
			"angle_between",
			lua.create_function(|_, args: (LuaDVec2, LuaDVec2)| {
				Ok((args.1.0.y - args.0.0.y).atan2(args.1.0.x - args.0.0.x))
			})?,
		)?;
		globals.set(
			"distance_between",
			lua.create_function(|_, args: (LuaDVec2, LuaDVec2)| Ok(args.1.0.distance(args.0.0)))?,
		)?;
		globals.set(
			"move_towards",
			lua.create_function(|_, args: (LuaDVec2, LuaDVec2, Number)| {
				Ok(LuaDVec2(args.0.0.move_towards(args.1.0, args.2)))
			})?,
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

fn lua_attack_fns(lua: &Lua) -> EvoidResult<Table> {
	let attacks = lua.create_table()?;

	attacks.set(
		"physical",
		lua.create_function(
			|_, (damage, size, pos, target, key): (_, _, LuaDVec2, LuaDVec2, String)| {
				Ok(Attack::new_physical(
					Obj::new(pos.0, target.0, size),
					damage,
					Owner::Enemy,
					&key,
				))
			},
		)?,
	)?;
	attacks.set(
		"burst",
		lua.create_function(|_, (damage, size, pos, key): (_, _, LuaDVec2, String)| {
			Ok(Attack::new_burst(
				Obj::new(pos.0, pos.0, size),
				damage,
				Owner::Enemy,
				&key,
			))
		})?,
	)?;
	attacks.set(
		"projectile",
		lua.create_function(
			|_, (damage, size, pos, target, key): (_, _, LuaDVec2, LuaDVec2, String)| {
				Ok(Attack::new_projectile(
					Obj::new(pos.0, target.0, size),
					damage,
					Owner::Enemy,
					&key,
				))
			},
		)?,
	)?;
	attacks.set(
		"hitscan",
		lua.create_function(
			|_, (damage, size, pos, target, key): (_, _, LuaDVec2, LuaDVec2, String)| {
				Ok(Attack::new_hitscan(
					Obj::new(pos.0, target.0, size),
					damage,
					Owner::Enemy,
					&key,
				))
			},
		)?,
	)?;

	Ok(attacks)
}
