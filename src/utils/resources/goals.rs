use rhai::AST;

use crate::cores::goal::get_goals;

use super::{Resource, get_resource_ref, resource};

static GOALS: Resource<AST> = resource();

pub(super) fn create_goals() {
	let mut access = GOALS.write();
	access.clear();
	*access = get_goals();
}

pub fn access_goal(key: &str) -> Option<&AST> {
	get_resource_ref(&GOALS, key)
}
