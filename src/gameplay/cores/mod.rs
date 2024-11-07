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

	// Handling subdirectories
	loop {
		let mut subdirs = false; // Checks if there could be further subdirectories

		for i in files.clone().iter().enumerate() {
			let dir = fs::read_dir(&i.1);

			// Checks if the directory is currently pointing to a file
			if let Err(_) = dir {
				continue
			}

			for j in dir.unwrap() {
				match j {
					Ok(dir) => {
						files.push(i.1.clone().to_owned() + "/" + dir.file_name().to_str().unwrap());
						subdirs = true;
					},
					_ => ()
				}
			}

			if subdirs {
				// The path at this index leads to a directory, so we do not want it. 
				files.remove(i.0);
			}
		}

		if !subdirs { break }
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
