use crate::cores::map::{Map, get_maps};

use super::{Resource, get_resource_ref, resource};

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
	get_resource_ref(&MAPS, key).unwrap() // TODO: Replace unwrap with proper error handling
}
