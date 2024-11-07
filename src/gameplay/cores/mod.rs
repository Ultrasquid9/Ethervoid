use std::fs;

pub mod attackscript;
pub mod enemytype;
pub mod map;
pub mod textures;

/// Creates a vec of Strings containing the directories of all of the provided files type in all cores
pub fn get_files(file_type: String) -> Vec<String> {
	// This function took way too long to write
	
	let mut files: Vec<String> = Vec::new(); // The complete directory of a file

	let mut paths: Vec<String> = Vec::new(); // The paths of different cores

	for i in fs::read_dir("./cores").unwrap() {
		let dir = i.unwrap().file_name().to_string_lossy().into_owned();
		paths.push(dir);
	}

	for i in paths {
		for j in fs::read_dir(format!("./cores/{}/{}", i, file_type).as_str()).unwrap() {
			files.push(format!("./cores/{}/{}/{}", i, file_type, j.unwrap().file_name().to_string_lossy().into_owned()));
		}
	}

	return files;
}

/// Turns the provided directory into a name
fn gen_name(dir: &str) -> String {
	let split: Vec<&str> = dir.split(&['/', '\\', '.'][..]).collect();

	return format!("{}:{}", split[3], {
		let mut str = String::new();

		for i in 3..split.len() - 1 {
			str.push_str(split[i]);
		}

		str
	});
}
