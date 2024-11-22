use std::sync::{LazyLock, RwLock};

use ahash::HashMap;

use crate::cores::map::{get_maps, Map};

static MAPS: LazyLock<RwLock<HashMap<String, Map>>> = LazyLock::new(|| return RwLock::new(HashMap::default()));

/// Populates the map HashMap
pub(super) fn create_maps() {
	let maps = get_maps();

	for i in maps {
		MAPS.write().unwrap().insert(i.0, i.1);
	}
}

/// Cleans the map HashMap
pub(super) fn clean_maps() {
	MAPS.write().unwrap().clear()
}

/// Gets the map at the provided key
pub fn access_map(key: &str) -> Map {
	MAPS.read().unwrap().get(key).unwrap().clone()
}
