use std::sync::LazyLock;

use tracing::error;

use crate::{
	cores::map::{Map, get_maps},
	gameplay::draw::process::to_texture,
};

use super::{Resource, get_resource_ref, resource, set_resource, textures::access_image};

/*
 * Maps
 */

static ERR_MAP: LazyLock<Map> = LazyLock::new(init_err_map);
static MAPS: Resource<Map> = resource();

/// Populates the map HashMap
pub(super) fn create_maps() {
	set_resource(&MAPS, get_maps());
}

/// Gets the map at the provided key
pub fn access_map(key: &str) -> &Map {
	match get_resource_ref(&MAPS, key) {
		Some(map) => map,
		None => {
			error!("Map {key} not found");
			&ERR_MAP
		}
	}
}

fn init_err_map() -> Map {
	// Shortcut for creating boxed slice
	fn b<T>() -> Box<[T]> {
		Box::new([])
	}

	Map {
		walls: b(),
		doors: b(),
		enemies: b(),
		npcs: b(),
		texture: to_texture(access_image("")),
	}
}
