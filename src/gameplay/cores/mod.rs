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
			if let Err(_) = dir { continue }

			for j in dir.unwrap() {
				// I have no clue what kind of error could occur here.
				// If there is one, then that is a problem for future me. 
				let dir = j.ok().unwrap(); 

				files.push(i.1.clone().to_owned() + "/" + dir.file_name().to_str().unwrap());
						
				if let Ok(_) = fs::read_dir(files[files.len() - 1].clone()) {
					subdirs = true;
				}
			}

			// The directory at the index has already been scanned, so it is no longer needed
			println!("Removing {}", i.1);
			files.remove(i.0);
		}

		if !subdirs { 
			for i in files.clone().iter().enumerate() {
				if let Ok(_) = fs::read_dir(i.1) {
					// Apparently not all directories are removed from the files list so I have to do an extra check
					files.remove(i.0);
				} else {
					println!("{}", i.1);
				}
			}
			break 
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
