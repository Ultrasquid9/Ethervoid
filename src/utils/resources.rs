use std::sync::LazyLock;

use ahash::HashMap;
use audio::create_sounds;

use goals::create_goals;
use maps::create_maps;

use parking_lot::RwLock;
use textures::create_textures;

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

/// Populates global resources, removing ones that were previously present.
pub fn create_resources() {
	create_sounds();
	create_textures();
	create_goals();
	create_maps();
}
