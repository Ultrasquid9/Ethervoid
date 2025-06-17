use mlua::{Compiler, Table, Value};
use rustc_hash::FxHashMap;
use tracing::{error, info, warn};

use crate::utils::{error::EvoidResult, resources::scripts::access_script};

use super::{gen_name, get_files};

/// A Lua script that can be used for advanced configuration of behavior
#[derive(Clone)]
pub struct Script {
	pub name: String,
	pub value: Value,
}

impl Script {
	pub fn new(key: &str) -> mlua::Result<Self> {
		Ok(Self {
			name: key.to_owned(),
			value: access_script(key)?.clone(),
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
pub fn get_scripts() -> FxHashMap<String, Vec<u8>> {
	let compiler = Compiler::new();

	get_files("scripts")
		.iter()
		.map(|dir| {
			let maybe_val = || Ok(compiler.compile(std::fs::read_to_string(dir)?)?);

			(gen_name(dir), maybe_val())
		})
		.filter_map(
			|(name, result): (String, EvoidResult<Vec<u8>>)| match result {
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
