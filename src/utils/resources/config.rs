use ron::ser;
use tracing::error;

use crate::{
	data::config::Config,
	utils::resources::{global, Global, GlobalAccess},
};

// TODO: Make more configurable.
// Maybe use system's config dir? Or use env variable?
const CONF_DIR: &str = "./config.ron";

static CONFIG: Global<Config> = global!(Config::read(CONF_DIR));

/// Reads the config file
pub fn read_config() -> Config {
	Config::read(CONF_DIR)
}

/// Gets the current config
pub fn access_config() -> GlobalAccess<Config> {
	CONFIG.read()
}

/// Updates and saves the config
pub fn update_config(cfg: Config) {
	let result = ser::to_string_pretty(&cfg, ser::PrettyConfig::new().indentor("\t"));
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
