use ahash::HashMap;
use engine::init_engine;
use tracing::{info, warn};

use crate::utils::{error::EvoidResult, resources::goals::access_goal};

use rhai::{AST, Engine, Scope};

use super::{gen_name, get_files};

mod engine;

/// A goal that can be configured via a script.
pub struct Goal {
	pub name: String,
	pub script: AST,
	pub scope: Scope<'static>,
	pub engine: Engine,
}

impl Goal {
	pub fn new(key: &str) -> Option<Goal> {
		Some(Goal {
			name: key.to_owned(),
			script: access_goal(key)?.clone(),
			scope: Scope::new(),
			engine: init_engine(),
		})
	}
}

impl Clone for Goal {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			script: self.script.clone(),
			scope: self.scope.clone(),
			engine: engine::init_engine(),
		}
	}
}

/// Provides a HashMap containing all Goals
pub fn get_goals() -> HashMap<String, AST> {
	let engine = init_engine();
	let goals = get_files("goals")
		.iter()
		.map(|dir| {
			let maybe_ast = || Ok(engine.compile(std::fs::read_to_string(dir)?)?);
			(gen_name(dir), maybe_ast())
		})
		.filter_map(|(name, result): (String, EvoidResult<AST>)| match result {
			Err(e) => {
				warn!("Failed to compile goal {name}: {e}");
				None
			}
			Ok(ast) => {
				info!("Goal {name} compiled!");
				Some((name, ast))
			}
		})
		.collect();

	goals
}
