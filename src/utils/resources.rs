use std::sync::LazyLock;

use ahash::HashMap;
use audio::{clean_sounds, create_sounds};

use goals::{clean_goals, create_goals};
use maps::{clean_maps, create_maps};

use parking_lot::RwLock;
use textures::{clean_textures, create_textures};

pub mod audio;
pub mod goals;
pub mod maps;
pub mod textures;

// This module contains globally available resources
// Everyone always says "don't do this" so fuck you I did

/// Stores a globally available resource
type Resource<T> = LazyLock<RwLock<HashMap<String, T>>>;

/// Creates a blank resource
const fn resource<T>() -> Resource<T> {
	LazyLock::new(|| RwLock::new(HashMap::default()))
}

/**
Populates global resources.

NOTE: Please ensure you call `clean_resources()` when quitting the game.
 */
pub unsafe fn create_resources() {
	create_sounds();
	create_textures();
	create_goals();
	create_maps();
}

/**
Cleans the global resources.

NOTE: THIS DELETES ALL RESOURCES. ONLY CALL WHEN QUITTING THE GAME.
 */
pub unsafe fn clean_resources() {
	clean_sounds();
	clean_textures();
	clean_goals();
	clean_maps();
}
