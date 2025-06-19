use mlua::{Lua, Number, Table, Value, Variadic};
use tracing::{debug, error, info, trace, warn};

use crate::{
	gameplay::{
		combat::{Attack, Owner},
		ecs::obj::Obj,
	},
	utils::{
		angle_between, delta_time,
		error::EvoidResult,
		mouse_pos, mouse_pos_local,
		resources::{audio::play_random_sound, scripts::access_script},
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

	attacks.set("_attacks", lua.create_table()?)?;
	attacks.set("spawn", lua.create_function(|lua, atk: Attack| {
		lua.globals().get::<Table>("attack")?.get::<Table>("_attacks")?.push(atk)
	})?)?;

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
		lua_fn!(lua, |args: Variadic<Number>| {
			use macroquad::rand::gen_range as rng;

			match args[..] {
				[] => rng(0., 1.),
				[a] => rng(1., a),
				[a, b] | [a, b, ..] => rng(a, b),
			}
		}),
	)?;

	// Since RNG is handled by Macroquad, this function would do nothing anyways
	math.set("randomseed", Value::Nil)?;

	Ok(())
}

fn lua_use_fns(lua: &Lua) -> EvoidResult<()> {
	let globals = lua.globals();

	globals.set(
		"use",
		lua_fn!(lua, |arg: String| access_script(&arg)?.clone()),
	)?;

	globals.set("require", Value::Nil)?;
	globals.set("package", Value::Nil)?;

	Ok(())
}

fn lua_log_fns(lua: &Lua) -> EvoidResult<()> {
	let globals = lua.globals();
	let log = lua.create_table()?;

	macro_rules! log {
		($log:ident) => {
			log.set(
				stringify!($log),
				lua_fn!(lua, |args: Variadic<Value>| $log!(
					"{}",
					stringify_args(args)
				)),
			)?
		};
	}

	log!(info);
	log!(warn);
	log!(error);
	log!(debug);
	log!(trace);

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
			play_random_sound(&args);
		}),
	)?;

	lua.globals().set("engine", engine)?;

	Ok(())
}

fn stringify_args(args: Variadic<Value>) -> String {
	args.into_iter()
		.map(|value| match value {
			Value::String(str) => str.to_string_lossy(),
			val => ron::to_string(&val).unwrap(),
		})
		.collect::<String>()
}
