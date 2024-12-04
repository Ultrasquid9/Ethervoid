use std::fs;
use ahash::HashMap;
use serde::Deserialize;
use macroquad::math::Vec2;

use crate::{
	gameplay::{
		combat::{
			Attack, 
			Owner
		}, 
		ecs::obj::Obj
	}, 
	utils::get_delta_time
};

use rhai::{
	Dynamic, 
	Engine, 
	EvalAltResult, 
	FnPtr, 
	NativeCallContext, 
	Scope
};

use super::{
	gen_name, 
	get_files, 
};

#[derive(Clone, Deserialize)]
pub struct ScriptBuilder(String);

/**
A behavior that can be configured via a script.

The lifetime annotation allows the compiler to know that the Script lives as long as its owner does
 */
pub struct Script {
	pub script: String,
	pub scope: Scope<'static>,
	pub engine: Engine
}


impl ScriptBuilder {
	/// Reads the script at the provided directory
	pub fn from(dir: String) -> Self {
		Self(fs::read_to_string(dir).unwrap())
	}

	/// Creates all the neccessary components for the script 
	pub fn build<'a>(self) -> Script {
		Script {
			script: self.0,
			scope: Scope::new(),
			engine: init_engine()
		}
	}
}

impl PartialEq for Script {
	fn eq(&self, other: &Self) -> bool { self.script == other.script }
}

impl Clone for Script {
	fn clone(&self) -> Self {
		Self {
			script: self.script.clone(),
			scope: self.scope.clone(),
			engine: init_engine()
		}
	}
}

/// Provides a HashMap containing all Attacks
pub fn get_scripts() -> HashMap<String, ScriptBuilder> {
	let mut attacks: HashMap<String, ScriptBuilder> = HashMap::default();

	for i in get_files(String::from("behavior")) {
		attacks.insert(
			gen_name(&i),
			ScriptBuilder::from(i)
		);
	}

	attacks
}

fn init_engine() -> Engine {
	let mut engine = Engine::new();

	// Some Vec2 built-in methods don't work, since Rhai methods dissallow immutable references.
	// As such, I have to make shitty copies.
	fn getter_x(pos: &mut Vec2) -> f32 { pos.x }
	fn getter_y(pos: &mut Vec2) -> f32 { pos.x }
	fn setter_x(pos: &mut Vec2, new: f32) { pos.x = new }
	fn setter_y(pos: &mut Vec2, new: f32) { pos.x = new }

	fn move_towards(pos1: &mut Vec2, pos2: Vec2, distance: f32) -> Vec2 {
		pos1.move_towards(pos2, distance)
	}
	fn distance_between(pos1: &mut Vec2, pos2: Vec2) -> f32 {
		pos1.distance(pos2)
	}
	fn angle_between(pos1: &mut Vec2, pos2: Vec2) -> f32 {
		(pos2.y - pos1.y).atan2(pos2.x - pos1.x)
	}

	engine
		// Registerring the Vec2 and functions related to it
		.register_type_with_name::<Vec2>("position")
		.register_get_set("x", getter_x, setter_x)
		.register_get_set("y", getter_y, setter_y)
		
		.register_fn("angle_between", angle_between)
		.register_fn("move_towards", move_towards)
		.register_fn("distance_between", distance_between)

		// Delta time
		.register_fn("delta", get_delta_time)

		// Functions for creating attacks
		.register_fn("new_physical", |
			damage: f32,
			size,
			pos: Vec2,
			target: Vec2,
			key: &str
		| Attack::new_physical(
			Obj::new(pos, target, size),
			damage, 
			Owner::Enemy,
			key
		))

		.register_fn("new_burst", |
			damage: f32,
			size: f32,
			pos: Vec2,
			key: &str
		| Attack::new_burst(
			Obj::new(pos, pos, size), 
			damage, 
			Owner::Enemy,
			key
		))

		.register_fn("new_projectile", |
			damage: f32,
			pos: Vec2,
			target: Vec2,
			key: &str
		| Attack::new_projectile(
			Obj::new(pos, target, 10.),
			damage, 
			Owner::Enemy,
			key
		))

		.register_fn("new_hitscan", |
			damage: f32,
			pos: Vec2,
			target: Vec2,
			_key: &str
		| Attack::new_hitscan(
			Obj::new(pos, target, 6.),
			damage, 
			Owner::Enemy
		))
		
		// Hacky method to end the script
		.register_fn("end", || Vec2::new(999999., 999999.))

		// Pipeline operator
		// IDK if this will ever be used, I just added it for fun 
		.register_custom_operator("|>", 255).unwrap()
		.register_fn("|>", |context: NativeCallContext, input: Dynamic, mut func: FnPtr| -> Result<Dynamic, std::boxed::Box<EvalAltResult>> {
			let mut curried = false;
			let mut args = func.curry().to_vec();

			for arg in args.iter_mut() {
				if !arg.is_char() { continue }

				if arg.clone_cast::<char>() == '_' {
					*arg = input.clone();
					curried = true;
				}
			}

			func.set_curry(Vec::new());
			func.call_within_context(&context, if curried {
				args
			} else { 
				let mut vec = Vec::new();
				vec.push(input);
				vec
			})
		})

		// Custom syntax for setting a variable if it does not already exist
		.register_custom_syntax([ "permanent", "$ident$", "<-", "$expr$" ], true, |context, inputs| {
			let var_name = inputs[0].get_string_value().unwrap().to_string();
			let value = context.eval_expression_tree(&inputs[1])?;

			if !context.scope().contains(&var_name) {
				context.scope_mut().push(var_name, value);
			}
			Ok(Dynamic::UNIT)
		}).unwrap();

	engine
}
