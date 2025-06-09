use ahash::HashMap;
use mlua::Table;
use tracing::{info, warn};

use crate::utils::{error::EvoidResult, resources::goals::{access_goal, lua}};

use super::{gen_name, get_files};

/// A goal that can be configured via a Lua script.
#[derive(Clone)]
pub struct Goal {
	pub name: String,
	pub table: Table,
}

impl Goal {
	pub fn new(key: &str) -> Option<Self> {
		Some(Self { 
			name: key.to_owned(), 
			table: access_goal(key)?.clone()
		})
	}
}

/// Provides a `HashMap` containing all Goals
pub fn get_goals() -> HashMap<String, Table> {
	let lua = lua();

	get_files("goals")
		.iter()
		.map(|dir| {
			let maybe_ast = || {
				Ok(lua.load(std::fs::read_to_string(dir)?).eval()?)
			};
		
			(gen_name(dir), maybe_ast())
		})
		.filter_map(|(name, result): (String, EvoidResult<Table>)| match result {
			Err(e) => {
				warn!("Failed to compile goal {name}: {e}");
				None
			}
			Ok(ast) => {
				info!("Goal {name} compiled!");
				Some((name, ast))
			}
		})
		.collect()
}
