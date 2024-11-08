use std::fs::{self, read_dir};

use walkdir::WalkDir;

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
			let dir = format!("./cores/{}/{}/{}", i, file_type, j.unwrap().file_name().to_string_lossy().into_owned());

			// Checking whether `dir` is a directory or a file
			if let Err(_) = fs::read_dir(&dir) {
				files.push(dir);
			} else { // Handling subdirectories
				let mut dirs = Vec::new();

				for entry in WalkDir::new(&dir) {
					dirs.push(entry.as_ref().unwrap().path().to_string_lossy().into_owned());
				}

				// Removing "leftover" entries
				dirs.retain(|dir| {
					if let Err(_) = read_dir(dir) {
						 if fs::exists(dir).unwrap() {
							return true
						}
					}
					return false
				});

				files.append(&mut dirs);
			}
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
