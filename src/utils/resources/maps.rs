use crate::cores::map::{Map, get_maps};

use super::{Resource, resource};

/*
 * Maps
 */

static MAPS: Resource<Map> = resource();

/// Populates the map HashMap
pub(super) fn create_maps() {
	let mut access = MAPS.write();
	access.clear();
	*access = get_maps();
}

/// Gets the map at the provided key
pub fn access_map(key: &str) -> &Map {
	// Raw pointer fuckery is here to allow returning a reference
	// This is an entirely pointless optimization, I just did it because I wanted to
	unsafe { (*MAPS.data_ptr()).get(key).unwrap() }
}
