use ahash::HashMap;
use parking_lot::RwLock;
use std::sync::LazyLock;

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
		MAPS.write().insert(key, map);
	}
}

/// Cleans the map HashMap
pub(super) fn clean_maps() {
	MAPS.write().clear()
}

/// Gets the map at the provided key
pub fn access_map(key: &str) -> &Map {
	// Raw pointer fuckery is here to allow returning a reference 
	// This is an entirely pointless optimization, I just did it because I wanted to 
	unsafe { (*MAPS.data_ptr()).get(key).unwrap() }
}
