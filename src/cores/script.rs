use std::fs;
use ahash::HashMap;
use serde::Deserialize;

use macroquad::math::Vec2;

use rhai::{
	Engine, 
	Scope
};

use super::{
	gen_name, 
	get_files, 
};

#[derive(Clone, Deserialize)]
pub struct ScriptBuilder(String);

impl ScriptBuilder {
	/// Creates an attack with the script at the provided directory
	pub fn from(dir: String) -> Self {
		return Self(fs::read_to_string(dir).unwrap())
	}

	/// Creates an attack with the script at the provided directory
	pub fn build<'a>(self) -> Script<'a> {
		return Script {
			current_target: Vec2::new(0., 0.),
			script: self.0,
			scope: Scope::new(),
			engine: Engine::new()
		}
	}
}

/// A behavior that can be configured via a script
/// The lifetime annotation allows the compiler to know that the Script lives as long as its owner does
pub struct Script<'a> {
	pub current_target: Vec2,
	script: String,
	scope: Scope<'a>,
	engine: Engine
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

	return attacks;
}
