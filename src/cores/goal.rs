use ahash::HashMap;
use serde::Deserialize;
use std::fs;

use crate::{
	gameplay::{
		combat::{Attack, Owner},
		ecs::obj::Obj,
	},
	prelude::*,
	utils::get_delta_time,
};

use rhai::{AST, Dynamic, Engine, EvalAltResult, FnPtr, NativeCallContext, Scope};

use super::{gen_name, get_files};

#[derive(Clone, Deserialize)]
pub struct GoalBuilder(String);


/// A goal that can be configured via a script.
pub struct Goal {
	pub script: AST,
	pub scope: Scope<'static>,
	pub engine: Engine,
}

impl GoalBuilder {
	/// Reads the script at the provided directory
	pub fn read(dir: &str) -> Self {
		Self(fs::read_to_string(dir).unwrap())
	}

	/// Creates all the neccessary components for the script
	pub fn build(self) -> Goal {
		let engine = init_engine();

		Goal {
			script: engine.compile(self.0).unwrap(),
			scope: Scope::new(),
			engine: init_engine(),
		}
	}
}

impl Clone for Goal {
	fn clone(&self) -> Self {
		Self {
			script: self.script.clone(),
			scope: self.scope.clone(),
			engine: init_engine(),
		}
	}
}

/// Provides a HashMap containing all Goals
pub fn get_goals() -> HashMap<String, GoalBuilder> {
	let goals: HashMap<String, GoalBuilder> = get_files("goals".to_string())
		.par_iter()
		.map(|dir| (gen_name(dir), GoalBuilder::read(dir)))
		.collect();

	goals
}

fn init_engine() -> Engine {
	let mut engine = Engine::new();

	// Some DVec2 built-in methods don't work, since Rhai methods dissallow immutable references.
	// As such, I have to make shitty copies.
	fn getter_x(pos: &mut DVec2) -> f64 {
		pos.x
	}
	fn getter_y(pos: &mut DVec2) -> f64 {
		pos.x
	}
	fn setter_x(pos: &mut DVec2, new: f64) {
		pos.x = new
	}
	fn setter_y(pos: &mut DVec2, new: f64) {
		pos.x = new
	}

	fn move_towards(pos1: &mut DVec2, pos2: DVec2, distance: f64) -> DVec2 {
		pos1.move_towards(pos2, distance)
	}
	fn distance_between(pos1: &mut DVec2, pos2: DVec2) -> f64 {
		pos1.distance(pos2)
	}
	fn angle_between(pos1: &mut DVec2, pos2: DVec2) -> f64 {
		(pos2.y - pos1.y).atan2(pos2.x - pos1.x)
	}

	engine
		// Disabling "eval" (this was recommended by the Rhai docs)
		.disable_symbol("eval")

		// Registerring the DVec2 and functions related to it
		.register_type_with_name::<DVec2>("position")
		.register_get_set("x", getter_x, setter_x)
		.register_get_set("y", getter_y, setter_y)
		.register_fn("angle_between", angle_between)
		.register_fn("move_towards", move_towards)
		.register_fn("distance_between", distance_between)

		// Delta time
		.register_fn("delta", get_delta_time)

		// Functions for creating attacks
		.register_fn(
			"new_physical",
			|damage: f64, size, pos: DVec2, target: DVec2, key: &str| {
				Attack::new_physical(Obj::new(pos, target, size), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"new_burst",
			|damage: f64, size: f64, pos: DVec2, key: &str| {
				Attack::new_burst(Obj::new(pos, pos, size), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"new_projectile",
			|damage: f64, pos: DVec2, target: DVec2, key: &str| {
				Attack::new_projectile(Obj::new(pos, target, 10.), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"new_hitscan",
			|damage: f64, pos: DVec2, target: DVec2, _key: &str| {
				Attack::new_hitscan(Obj::new(pos, target, 6.), damage, Owner::Enemy)
			},
		)

		// Pipeline operator
		// IDK if this will ever be used, I just added it for fun
		.register_custom_operator("|>", 255)
		.unwrap()
		.register_fn(
			"|>",
			|context: NativeCallContext,
			 input: Dynamic,
			 mut func: FnPtr|
			 -> std::result::Result<Dynamic, Box<EvalAltResult>> {
				let mut curried = false;
				let mut args = func.curry().to_vec();

				for arg in args.iter_mut() {
					if !arg.is_char() {
						continue;
					}

					if arg.clone_cast::<char>() == '_' {
						*arg = input.clone();
						curried = true;
					}
				}

				func.set_curry(Vec::new());
				func.call_within_context(
					&context,
					if curried {
						args
					} else {
						let mut vec = vec![input];
						vec.append(&mut args);
						vec
					},
				)
			},
		);

	engine
}
