use rhai::AST;

use crate::cores::goal::get_goals;

use super::{Resource, resource};

static GOALS: Resource<AST> = resource();

pub(super) fn create_goals() {
	*GOALS.write() = get_goals();
}

pub(super) fn clean_goals() {
	(*GOALS.write()).clear();
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
