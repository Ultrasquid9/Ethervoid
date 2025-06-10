use std::sync::LazyLock;

use ahash::HashMap;
use audio::create_sounds;

use maps::create_maps;
use script_vals::create_script_vals;
use tracing::info;

use parking_lot::RwLock;
use textures::create_textures;

pub mod audio;
pub mod config;
pub mod maps;
pub mod script_vals;
pub mod textures;

// This module contains globally available resources
// Everyone always says "don't do this" so fuck you I did

/// Stores a globally available resource
type Resource<T> = LazyLock<RwLock<HashMap<String, T>>>;

/// Creates a blank resource
const fn resource<T>() -> Resource<T> {
	LazyLock::new(|| RwLock::new(HashMap::default()))
}

/// Gets a reference to the item stored in the resource at the given key
fn get_resource_ref<'a, T>(resource: &'a Resource<T>, key: &str) -> Option<&'a T> {
	// Raw pointer fuckery is here to allow returning a reference instead of cloning.
	//
	// Safely reading from a RwLock is slightly expensive and doesn't play well with references,
	// so this instead gets a raw pointer to the inner data and immediately dereferences it.
	unsafe { (*resource.data_ptr()).get(key) }
}

/// Clears a resource and sets it to the provided data
fn set_resource<T>(resource: &Resource<T>, data: HashMap<String, T>) {
	let mut access = resource.write();
	access.clear();
	*access = data;
}

/// Populates global resources, removing ones that were previously present.
pub fn create_resources() {
	std::thread::scope(|scope| {
		scope.spawn(create_textures);
		scope.spawn(create_sounds);
		scope.spawn(create_script_vals);
	});
	create_maps(); // Maps depend on the existance of the other resources
	info!("All resources loaded!");
}
