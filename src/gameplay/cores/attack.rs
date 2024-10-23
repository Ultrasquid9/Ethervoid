use std::collections::HashMap;

use macroquad::math::Vec2;

use super::{get_files, get_name};

#[derive(Clone)]
pub struct Attack {
	current_target: Vec2,
	dir: String
}

impl Attack {
	pub fn from(dir: String) -> Self {
		Attack {
			current_target: Vec2::new(0., 0.),
			dir
		}
	}
}

/// Provides a HashMap containing all Attacks
pub fn get_attacks() -> HashMap<String, Attack> {
	let mut attacks: HashMap<String, Attack> = HashMap::new();

	for i in get_files(String::from("attacks")) {
		attacks.insert(
			get_name(&i),
			Attack::from(i)
		);
	}

	return attacks;
}
