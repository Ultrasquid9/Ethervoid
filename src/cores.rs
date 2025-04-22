use std::fs::{self, read_dir};

use tracing::error;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::utils::error::Result;

pub mod audio;
pub mod enemytype;
pub mod goal;
pub mod map;
pub mod npctype;
pub mod textures;

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

	for result in read_dir("./cores").expect("Cores directory should exist") {
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

/// Turns the provided directory into a name
fn gen_name(dir: &str) -> String {
	let split: Vec<&str> = dir.split(&['/', '\\', '.'][..]).collect();

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

pub trait Readable {
	fn read(dir: &str) -> Result<Self>
	where
		Self: Sized,
		Self: for<'a> Deserialize<'a>,
	{
		let file = fs::read_to_string(dir)?;
		Ok(ron::from_str(&file)?)
	}
}
