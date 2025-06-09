use std::sync::LazyLock;

use mlua::{Lua, Table};

use crate::{cores::goal::get_goals, utils::lua::create_lua};

use super::{Resource, get_resource_ref, resource, set_resource};

static GOALS: Resource<Table> = resource();
static LUA: LazyLock<Lua> = LazyLock::new(create_lua);

pub(super) fn create_goals() {
	set_resource(&GOALS, get_goals());
}

pub fn access_goal(key: &str) -> Option<&Table> {
	get_resource_ref(&GOALS, key)
}

pub fn lua() -> &'static Lua {
	&LUA
}
