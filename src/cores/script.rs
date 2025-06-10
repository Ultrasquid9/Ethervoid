use ahash::HashMap;
use mlua::{Compiler, Table, Value};
use tracing::{error, info, warn};

use crate::utils::{
	error::EvoidResult,
	resources::script_vals::{access_script_val, lua},
};

use super::{gen_name, get_files};

/// A Lua script that can be used for advanced configuration of behavior
#[derive(Clone)]
pub struct Script {
	pub name: String,
	pub value: Value,
}

impl Script {
	pub fn new(key: &str) -> Option<Self> {
		Some(Self {
			name: key.to_owned(),
			value: access_script_val(key)?.clone(),
		})
	}

	pub fn table(&self) -> EvoidResult<Table> {
		if let Some(table) = self.value.as_table() {
			Ok(table.clone())
		} else {
			error!("Expected {} to contain table", self.name);
			Err(mlua::Error::FromLuaConversionError {
				from: self.value.type_name(),
				to: "Table".into(),
				message: None,
			}
			.into())
		}
	}
}

/// Provides a `HashMap` containing all Script values
pub fn get_script_vals() -> HashMap<String, Value> {
	let lua = lua();
	let compiler = Compiler::new();

	get_files("scripts")
		.iter()
		.map(|dir| {
			let maybe_val = || {
				let bytecode = compiler.compile(std::fs::read_to_string(dir)?)?;
				Ok(lua.load(bytecode).eval()?)
			};

			(gen_name(dir), maybe_val())
		})
		.filter_map(
			|(name, result): (String, EvoidResult<Value>)| match result {
				Err(e) => {
					warn!("Failed to compile script {name}: {e}");
					None
				}
				Ok(val) => {
					info!("Script {name} compiled!");
					Some((name, val))
				}
			},
		)
		.collect()
}
