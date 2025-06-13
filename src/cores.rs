use std::{
	fs::{self, read_dir},
	path::{Path, PathBuf},
	time::SystemTime,
};

use hashbrown::HashMap;
use serde::Deserialize;
use tracing::{error, warn};
use walkdir::WalkDir;

use crate::utils::{
	error::EvoidResult,
	resources::{Global, global},
};

pub mod audio;
pub mod enemytype;
pub mod lang;
pub mod map;
pub mod npctype;
pub mod script;
pub mod textures;

const DIR_SPLIT: &[char] = &['/', '\\', '.'];
const CORES_DIR: &str = "./cores"; // TODO: Make Configurable 

static DIR_CACHE: Global<HashMap<PathBuf, SystemTime>> = global!(HashMap::new());

/// Creates a vec of Strings containing the directories of all of the provided files type in all cores
pub fn get_files(file_type: &str) -> Vec<String> {
	macro_rules! maybe {
		($result:expr) => {
			match $result {
				Ok(ok) => ok,
				Err(e) => {
					error!("{e}");
					continue;
				}
			}
		};
	}

	// This function took way too long to write

	let mut files = vec![]; // The complete directory of a file
	let mut paths = vec![]; // The paths of different cores

	for result in read_dir(CORES_DIR).expect("Cores directory should exist") {
		paths.push(maybe!(result).file_name().to_string_lossy().into_owned());
	}

	for path in paths {
		let iter = maybe!(read_dir(format!("./cores/{path}/{file_type}").as_str()));

		for result in iter {
			// The directory to be scanned
			let dir = maybe!(result).path().to_string_lossy().into_owned();
			// Directories that will be appended to `files` and returned
			let mut dirs = vec![];

			for result in WalkDir::new(&dir) {
				dirs.push(maybe!(result).path().to_string_lossy().into_owned());
			}

			// Removing "leftover" entries
			dirs.retain(|dir| read_dir(dir).is_err() && fs::exists(dir).unwrap_or(false));
			files.append(&mut dirs);
		}
	}

	files
}

/// Checks if any cores have changed since the last time this function was called
pub fn cores_changed() -> bool {
	let mut fs = HashMap::new();

	for result in WalkDir::new(CORES_DIR) {
		let path = match result {
			Ok(ok) => ok.into_path(),
			Err(e) => {
				warn!("{e}");
				return true;
			}
		};

		let modified = match fs::metadata(&path).and_then(|m| m.modified()) {
			Ok(ok) => ok,
			Err(e) => {
				warn!("Could not check if {path:?} was modified: {e}");
				return true;
			}
		};

		fs.insert(path, modified);
	}

	if *DIR_CACHE.read() == fs {
		false
	} else {
		*DIR_CACHE.write() = fs;
		true
	}
}

/// Attempts to read a RON file at the provided path
pub fn read_from_path<T>(dir: impl AsRef<Path>) -> EvoidResult<T>
where
	T: for<'a> Deserialize<'a>,
{
	Ok(ron::from_str(&fs::read_to_string(dir)?)?)
}

/// Turns the provided directory into a name
fn gen_name(dir: &str) -> String {
	let split: Vec<&str> = dir.split(DIR_SPLIT).collect();

	format!("{}:{}", split[3], {
		let mut str = String::new();

		for i in 5..split.len() - 1 {
			// Adds slashes in between subdirectories
			if i > 5 && i < split.len() - 1 {
				str.push('/');
			}
			str.push_str(split[i]);
		}

		str
	})
}
