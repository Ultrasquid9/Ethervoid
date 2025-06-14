use mlua::{Lua, Value};

use crate::{
	cores::script::get_script_vals,
	utils::{
		lua::create_lua,
		resources::{global, Global, GlobalAccess},
	},
};

use super::{Resource, get_resource_ref, resource, set_resource};

static SCRIPT_VALS: Resource<Value> = resource();
static LUA: Global<Lua> = global!(create_lua());

pub(super) fn create_script_vals() {
	// Resets the Lua instance, to avoid old stuff lingering around within it
	*LUA.write() = create_lua();

	set_resource(&SCRIPT_VALS, get_script_vals());
}

pub fn access_script_val(key: &str) -> Option<&Value> {
	get_resource_ref(&SCRIPT_VALS, key)
}

pub fn lua() -> GlobalAccess<Lua> {
	LUA.read()
}
