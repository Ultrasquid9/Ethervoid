use ahash::HashMap;
use log::{debug, error, info, trace, warn};
use mod_resolver::EvoidResolver;

use crate::{
	gameplay::{
		combat::{Attack, Owner},
		ecs::obj::Obj,
	},
	prelude::*,
	utils::{error::Result, get_delta_time, resources::goals::access_goal},
};

use rhai::{AST, Dynamic, Engine, EvalAltResult, FnPtr, NativeCallContext, Scope};

use super::{gen_name, get_files};

/// A goal that can be configured via a script.
pub struct Goal {
	pub name: String,
	pub script: AST,
	pub scope: Scope<'static>,
	pub engine: Engine,
}

impl Goal {
	pub fn new(key: &str) -> Goal {
		Goal {
			name: key.to_owned(),
			script: access_goal(key).unwrap(),
			scope: Scope::new(),
			engine: init_engine(),
		}
	}
}

impl Clone for Goal {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			script: self.script.clone(),
			scope: self.scope.clone(),
			engine: init_engine(),
		}
	}
}

/// Provides a HashMap containing all Goals
pub fn get_goals() -> HashMap<String, AST> {
	let engine = init_engine();
	let goals = get_files("goals".to_string())
		.par_iter()
		.map(|dir| {
			Ok((
				gen_name(dir),
				engine.compile(std::fs::read_to_string(dir)?)?,
			))
		})
		.filter_map(|result: Result<(String, AST)>| {
			if let Err(e) = result {
				warn!("Failed to compile goal: {e}");
				None
			} else {
				result.ok()
			}
		})
		.collect();

	goals
}

fn init_engine() -> Engine {
	// Output of custom operators
	type OperatorResult = std::result::Result<Dynamic, Box<EvalAltResult>>;

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

	let mut engine = Engine::new();
	engine
		// Allowing modules
		.set_module_resolver(EvoidResolver)
		// Disabling "eval" (this was recommended by the Rhai docs)
		.disable_symbol("eval")
		// Disabling throwing exceptions
		.disable_symbol("throw")
		// Altering the built-in print methods and adding the remaining log types
		.on_print(|s| info!("{s}"))
		.on_debug(|s: &str, _, _| debug!("{s}"))
		.register_fn("trace", |s: &str| trace!("{s}"))
		.register_fn("warn", |s: &str| warn!("{s}"))
		.register_fn("error", |s: &str| error!("{s}"))
		// Registerring the DVec2 and functions related to it
		.register_type_with_name::<DVec2>("position")
		.register_get_set("x", getter_x, setter_x)
		.register_get_set("y", getter_y, setter_y)
		.register_fn("position", |x: f64, y: f64| dvec2(x, y))
		.register_fn("angle_between", angle_between)
		.register_fn("move_towards", move_towards)
		.register_fn("distance_between", distance_between)
		// Delta time
		.register_fn("delta", get_delta_time)
		// Functions for creating attacks
		.register_fn(
			"attack_physical",
			|damage: f64, size, pos: DVec2, target: DVec2, key: &str| {
				Attack::new_physical(Obj::new(pos, target, size), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"attack_burst",
			|damage: f64, size: f64, pos: DVec2, key: &str| {
				Attack::new_burst(Obj::new(pos, pos, size), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"attack_projectile",
			|damage: f64, pos: DVec2, target: DVec2, key: &str| {
				Attack::new_projectile(Obj::new(pos, target, 10.), damage, Owner::Enemy, key)
			},
		)
		.register_fn(
			"attack_hitscan",
			|damage: f64, pos: DVec2, target: DVec2, _key: &str| {
				Attack::new_hitscan(Obj::new(pos, target, 6.), damage, Owner::Enemy)
			},
		)
		// Ternary operator
		// Uses an array for the input, since custom operators are somewhat limited
		.register_custom_operator("?", 131)
		.unwrap()
		.register_fn("?", |input: bool, array: Vec<Dynamic>| -> OperatorResult {
			let output = array.get(if input { 0 } else { 1 });

			let Some(output) = output else {
				return Ok(Dynamic::from(()));
			};

			return Ok(output.clone());
		})
		// Pipeline operator
		// IDK if this will ever be used, I just added it for fun
		.register_custom_operator("|>", 255)
		.unwrap()
		.register_fn(
			"|>",
			|context: NativeCallContext, input: Dynamic, mut func: FnPtr| -> OperatorResult {
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

				func.set_curry(vec![]);
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

mod mod_resolver {
	use std::sync::Arc;

	use rhai::{EvalAltResult, Module, ModuleResolver, Scope};

	use crate::utils::resources::goals::access_goal;

	pub struct EvoidResolver;

	impl ModuleResolver for EvoidResolver {
		fn resolve(
			&self,
			engine: &rhai::Engine,
			_source: Option<&str>,
			path: &str,
			pos: rhai::Position,
		) -> std::result::Result<Arc<Module>, Box<EvalAltResult>> {
			if let Some(ast) = access_goal(path) {
				return Ok(Arc::new(Module::eval_ast_as_new(
					Scope::new(),
					&ast,
					engine,
				)?));
			}

			Err(EvalAltResult::ErrorModuleNotFound(path.into(), pos).into())
		}
	}
}
