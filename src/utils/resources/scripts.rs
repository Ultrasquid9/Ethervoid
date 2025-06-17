use mlua::{Lua, Value};
use tracing::{error, info};

use crate::{
	cores::script::get_scripts,
	utils::{
		lua::create_lua,
		resources::{Global, GlobalAccess, global},
	},
};

use super::{Resource, get_resource_ref, resource, set_resource};

static SCRIPTS: Resource<Vec<u8>> = resource();
static EXECUTED_SCRIPTS: Resource<Value> = resource();

static LUA: Global<Lua> = global!(create_lua());

pub(super) fn create_script_vals() {
	// Resets the Lua instance, to avoid old stuff lingering around within it
	*LUA.write() = create_lua();

	set_resource(&SCRIPTS, get_scripts());
}

pub fn access_script(key: &str) -> mlua::Result<&Value> {
	if let Some(val) = get_resource_ref(&EXECUTED_SCRIPTS, key) {
		Ok(val)
	} else if let Some(bytecode) = SCRIPTS.read().get(key) {
		info!("Attempting to evaluate script {key}");

		let val: Value = lua().load(bytecode).eval()?;
		EXECUTED_SCRIPTS.write().insert(key.to_string(), val);

		access_script(key)
	} else {
		error!("Script {key} not found!");
		Ok(&Value::Nil)
	}
}

/// Gets access to the [Lua] instance
pub fn lua() -> GlobalAccess<Lua> {
	LUA.read()
}
