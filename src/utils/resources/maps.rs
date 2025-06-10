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

/// Populates the map `HashMap`
pub(super) fn create_maps() {
	set_resource(&MAPS, get_maps());
}

/// Gets the map at the provided key
pub fn access_map(key: &str) -> &Map {
	if let Some(map) = get_resource_ref(&MAPS, key) {
		map
	} else {
		error!("Map {key} not found");
		&ERR_MAP
	}
}

fn init_err_map() -> Map {
	Map {
		walls: [].into(),
		doors: [].into(),
		enemies: [].into(),
		npcs: [].into(),
		texture: to_texture(access_image("")),
	}
}
