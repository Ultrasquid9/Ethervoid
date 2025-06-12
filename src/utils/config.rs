use std::fs;

use keymap::KeyMap;
use tracing::error;

use serde::{Deserialize, Serialize};

pub mod keymap;

/// The config for the game
#[derive(Serialize, Deserialize)]
pub struct Config {
	pub keymap: KeyMap,

	/// The map where the game starts on a new save
	/// TODO: allow mods to configure this
	pub start_map: String,

	/// The language used by the game
	pub lang: String,
}

impl Config {
	/// Reads the config file
	pub fn read(dir: &str) -> Self {
		match ron::from_str(&match fs::read_to_string(dir) {
			Ok(str) => str,
			Err(e) => {
				error!("Error when reading config: {e}");
				return Self::default();
			}
		}) {
			Ok(config) => config,
			Err(e) => {
				error!("Error when deserializing config: {e}");
				Self::default()
			}
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			keymap: KeyMap::default(),

			start_map: "default:test".into(),
			lang: "en".into(),
		}
	}
}
