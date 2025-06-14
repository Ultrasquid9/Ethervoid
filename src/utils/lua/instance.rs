use mlua::{Lua, Number, Table, Value, Variadic};
use tracing::{debug, error, info, trace, warn};

use crate::{
	gameplay::{
		combat::{Attack, Owner},
		ecs::obj::Obj,
	},
	utils::{
		ImmutVec, angle_between, delta_time,
		error::EvoidResult,
		mouse_pos, mouse_pos_local,
		resources::{audio::play_random_sound, script_vals::access_script_val},
	},
};

use super::LuaDVec2;

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

pub fn try_create_lua() -> EvoidResult<Lua> {
	let lua = Lua::new();
	let globals = lua.globals();

	globals.set(
		"angle_between",
		lua_fn!(lua, |current: LuaDVec2, target: LuaDVec2| {
			angle_between(&target, &current)
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

	lua_attack_fns(&lua)?;
	lua_math_fns(&lua)?;
	lua_use_fns(&lua)?;
	lua_log_fns(&lua)?;
	lua_engine_fns(&lua)?;

	lua.sandbox(true)?;
	Ok(lua)
}

#[rustfmt::skip]
fn lua_attack_fns(lua: &Lua) -> EvoidResult<()> {
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

	lua.globals().set("attack", attacks)?;
	Ok(())
}

fn lua_math_fns(lua: &Lua) -> EvoidResult<()> {
	let math: Table = lua.globals().get("math")?;

	// Apparently Lua does not have a round function built-in, so I fixed that
	math.set("round", lua_fn!(lua, |num: Number| num.round()))?;

	// Making RNG be handled by Macroquad
	math.set(
		"random",
		lua.create_function(|_, args: Variadic<Number>| {
			use macroquad::rand::gen_range as rng;

			Ok(match args[..] {
				[] => rng(0., 1.),
				[a] => rng(1., a),
				[a, b] | [a, b, ..] => rng(a, b),
			})
		})?,
	)?;

	// Since RNG is handled by Macroquad, this function would do nothing anyways
	math.set("randomseed", Value::Nil)?;

	Ok(())
}

fn lua_use_fns(lua: &Lua) -> EvoidResult<()> {
	let globals = lua.globals();

	globals.set(
		"use",
		lua.create_function(|_, arg: String| {
			Ok(access_script_val(&arg).unwrap_or(&Value::Nil).clone())
		})?,
	)?;

	globals.set("require", Value::Nil)?;
	globals.set("package", Value::Nil)?;

	Ok(())
}

fn lua_log_fns(lua: &Lua) -> EvoidResult<()> {
	let globals = lua.globals();
	let log = lua.create_table()?;

	log.set("info", lua_fn!(lua, |arg: String| info!("{arg}")))?;
	log.set("warn", lua_fn!(lua, |arg: String| warn!("{arg}")))?;
	log.set("error", lua_fn!(lua, |arg: String| error!("{arg}")))?;
	log.set("debug", lua_fn!(lua, |arg: String| debug!("{arg}")))?;
	log.set("trace", lua_fn!(lua, |arg: String| trace!("{arg}")))?;

	globals.set("log", log)?;

	globals.set("print", Value::Nil)?;
	globals.set("io", Value::Nil)?;

	Ok(())
}

fn lua_engine_fns(lua: &Lua) -> EvoidResult<()> {
	let engine = lua.create_table()?;

	engine.set("delta_time", lua_fn!(lua, delta_time))?;
	engine.set("mouse_pos", lua_fn!(lua, || LuaDVec2::from(mouse_pos())))?;
	engine.set(
		"mouse_pos_local",
		lua_fn!(lua, || LuaDVec2::from(mouse_pos_local())),
	)?;

	engine.set(
		"play_sound",
		lua_fn!(lua, |args: Variadic<String>| {
			play_random_sound(&args.iter().map(String::as_str).collect::<ImmutVec<&str>>());
		}),
	)?;

	lua.globals().set("engine", engine)?;

	Ok(())
}
