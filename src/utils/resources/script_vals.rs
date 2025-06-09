use std::sync::LazyLock;

use mlua::{Lua, Value};

use crate::{cores::script::get_script_vals, utils::lua::create_lua};

use super::{Resource, get_resource_ref, resource, set_resource};

static SCRIPT_VALS: Resource<Value> = resource();
static LUA: LazyLock<Lua> = LazyLock::new(create_lua);

pub(super) fn create_script_vals() {
	set_resource(&SCRIPT_VALS, get_script_vals());
}

pub fn access_script_val(key: &str) -> Option<&Value> {
	get_resource_ref(&SCRIPT_VALS, key)
}

pub fn lua() -> &'static Lua {
	&LUA
}
