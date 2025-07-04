use std::sync::LazyLock;

use audio::create_sounds;
use rustc_hash::FxHashMap;

use maps::create_maps;
use scripts::create_script_vals;
use tracing::info;

use parking_lot::{RwLock, RwLockReadGuard};
use textures::create_textures;

use crate::{cores::cores_changed, utils::resources::langs::create_langs};

pub mod audio;
pub mod config;
pub mod langs;
pub mod maps;
pub mod scripts;
pub mod textures;

// This module contains globally available resources
// Everyone always says "don't do this" so fuck you I did

/// Stores a globally available value
pub type Global<T> = LazyLock<RwLock<T>>;
/// Stores a globally available resource
type Resource<T> = Global<FxHashMap<String, T>>;
type GlobalAccess<T> = RwLockReadGuard<'static, T>;

/// Creates a blank resource
const fn resource<T>() -> Resource<T> {
	LazyLock::new(|| RwLock::new(FxHashMap::default()))
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
fn set_resource<T>(resource: &Resource<T>, data: FxHashMap<String, T>) {
	let mut access = resource.write();
	access.clear();
	*access = data;
}

/// Populates global resources, removing ones that were previously present.
pub fn create_resources() {
	if !cores_changed() {
		info!("Cores unchanged, reusing current resources");
		return;
	}

	rayon::scope(|s| {
		s.spawn(|_| create_textures());
		s.spawn(|_| create_sounds());
		s.spawn(|_| create_script_vals());
		s.spawn(|_| create_langs());
	});
	create_maps(); // Maps depend on the existance of the other resources
	info!("All resources loaded!");
}

/// Creates a [Global] with the provided data
macro_rules! global {
	($input:expr) => {
		std::sync::LazyLock::new(|| parking_lot::RwLock::new($input))
	};
}
pub(crate) use global;
