use std::fs;

use serde_json::Value;

pub mod enemybuilder;
pub mod mapbuilder;

/// Creates a vec of Strings containing the directories of all of the provided builder type in all cores
pub fn get_builders(builder_type: String) -> Vec<String> {
	// This function took way too long to write
	
	let mut builders: Vec<String> = Vec::new();

	let mut enemies_paths: Vec<String> = Vec::new();

	for i in fs::read_dir("./cores").unwrap() {
		let dir = i.unwrap().file_name().to_string_lossy().into_owned();
		enemies_paths.push(dir);
	}

	for i in enemies_paths {
		for j in fs::read_dir(format!("./cores/{}/{}", i, builder_type).as_str()).unwrap() {
			builders.push(format!("./cores/{}/{}/{}", i, builder_type, j.unwrap().file_name().to_string_lossy().into_owned()));
		}
	}

	return builders;
}

pub fn get_name(dir: &str) -> String {
	let input: Value = serde_json::from_str(&fs::read_to_string(dir).expect("File does not exist!")).unwrap();
	return input["Name"].as_str().unwrap().to_owned();
}