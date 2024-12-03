use ahash::HashMap;

use std::sync::{
	LazyLock, 
	RwLock
};

use crate::cores::map::{
	get_maps,
	Map
};

/*
 * Maps
 */

static MAPS: LazyLock<RwLock<HashMap<String, Map>>> = LazyLock::new(|| RwLock::new(HashMap::default()));

/// Populates the map HashMap
pub(super) fn create_maps() {
	let maps = get_maps();

	for (key, map) in maps {
		MAPS.write().unwrap().insert(key, map);
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
