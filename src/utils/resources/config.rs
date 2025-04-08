use std::sync::LazyLock;

use log::error;
use parking_lot::RwLock;

use crate::utils::config::Config;

// TODO: Make more configurable.
// Maybe use system's config dir? Or use env variable?
const CONF_DIR: &str = "./config.ron";

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::read(CONF_DIR)));

pub fn read_config() -> Config {
	Config::read(CONF_DIR)
}

/// Gets the current config
pub fn access_config() -> &'static Config {
	unsafe { &*CONFIG.data_ptr() }
}

/// Updates and saves the config
pub fn update_config(cfg: Config) {
	let result = ron::to_string(&cfg);
	*CONFIG.write() = cfg;

	let str = match result {
		Ok(str) => str,
		Err(e) => {
			error!("Config could not be serialized: {e}");
			return;
		}
	};

	if let Err(e) = std::fs::write(CONF_DIR, str) {
		error!("Config could not be written: {e}")
	}
}
