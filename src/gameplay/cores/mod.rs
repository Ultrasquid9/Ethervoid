use std::fs;

use macroquad::math::Vec2;
use serde::Deserialize;

pub mod enemytype;
pub mod map;
pub mod attackscript;

#[derive(Clone, Deserialize)]
pub struct Point(f32, f32);

impl Point {
	pub fn to_vec2(&self) -> Vec2 {
		Vec2::new(self.0, self.1)
	}
}

/// Creates a vec of Strings containing the directories of all of the provided files type in all cores
pub fn get_files(file_type: String) -> Vec<String> {
	// This function took way too long to write
	
	let mut files: Vec<String> = Vec::new();

	let mut enemies_paths: Vec<String> = Vec::new();

	for i in fs::read_dir("./cores").unwrap() {
		let dir = i.unwrap().file_name().to_string_lossy().into_owned();
		enemies_paths.push(dir);
	}

	for i in enemies_paths {
		for j in fs::read_dir(format!("./cores/{}/{}", i, file_type).as_str()).unwrap() {
			files.push(format!("./cores/{}/{}/{}", i, file_type, j.unwrap().file_name().to_string_lossy().into_owned()));
		}
	}

	return files;
}

/// Turns the provided directory into a name
fn gen_name(dir: &str) -> String {
	let split: Vec<&str> = dir.split(&['/', '\\', '.'][..]).collect();

	return format!("{}:{}", split[3], split[5]);
}
