use rhai::AST;

use crate::cores::goal::get_goals;

use super::{Resource, resource};

static GOALS: Resource<AST> = resource();

pub(super) fn create_goals() {
	let mut access = GOALS.write();
	access.clear();
	*access = get_goals();
}

pub fn goal_exists(key: &str) -> bool {
	GOALS.read().contains_key(key)
}

pub fn access_goal(key: &str) -> Option<AST> {
	if goal_exists(key) {
		Some(GOALS.read().get(key).unwrap().clone())
	} else {
		None
	}
}
