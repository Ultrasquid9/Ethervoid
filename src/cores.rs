use walkdir::WalkDir;

use std::fs::{
	self, 
	read_dir
};

pub mod script;
pub mod enemytype;

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
		if fs::read_dir(format!("./cores/{}/{}", i, file_type).as_str()).is_err() {
			continue
		} 

		for j in fs::read_dir(format!("./cores/{}/{}", i, file_type).as_str()).unwrap() {

			// The directory to be scanned
			//let dir = format!("./cores/{}/{}/{}", i, file_type, j.unwrap().file_name().to_string_lossy().into_owned());
			let dir = j.unwrap().path().to_string_lossy().into_owned();
			// Directories that will be appended to `files` and returned
			let mut dirs = Vec::new();

			for entry in WalkDir::new(&dir) {
				dirs.push(entry.as_ref().unwrap().path().to_string_lossy().into_owned());
			}

			// Removing "leftover" entries
			dirs.retain(|dir| {
				if read_dir(dir).is_err() && fs::exists(dir).unwrap() {
					return true
				}
				return false
			});

			files.append(&mut dirs);
		}
	}

	return files;
}

/// Turns the provided directory into a name
fn gen_name(dir: &str) -> String {
	let split: Vec<&str> = dir.split(&['/', '\\', '.'][..]).collect();

	return format!("{}:{}", split[3], {
		let mut str = String::new();

		for i in 5..split.len() - 1 {
			// Adds slashes in between subdirectories
			if i > 5 && i < split.len() - 1 {
				str.push('/');
			}
			str.push_str(split[i]);
		}

		str
	});
}
