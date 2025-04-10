use rhai::AST;

use crate::cores::goal::get_goals;

use super::{Resource, get_resource_ref, resource, set_resource};

static GOALS: Resource<AST> = resource();

pub(super) fn create_goals() {
	set_resource(&GOALS, get_goals());
}

pub fn access_goal(key: &str) -> Option<&AST> {
	get_resource_ref(&GOALS, key)
}
