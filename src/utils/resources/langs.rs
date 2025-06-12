use fluent::FluentArgs;
use tracing::{error, warn};

use crate::{
	cores::lang::{Lang, get_langs},
	utils::resources::{Resource, config::access_config, get_resource_ref, resource, set_resource},
};

/*
 * Languages
 */

static LANGS: Resource<Lang> = resource();

/// Populates the language `HashMap`
pub(super) fn create_langs() {
	set_resource(&LANGS, get_langs());
}

/// Gets the language value at the provided key
pub fn access_lang(key: &str) -> String {
	access_lang_with_args(key, &FluentArgs::new())
}

/// Gets the languaage value at the provided key, passing in the provided [`FluentArgs`]
pub fn access_lang_with_args(key: &str, args: &FluentArgs) -> String {
	let lang_key = &access_config().lang;

	if let Some(lang) = get_resource_ref(&LANGS, lang_key) {
		if let Some(msg) = lang.get_message(key) {
			let mut warnings = vec![];

			let out = lang
				.format_pattern(msg.value().unwrap(), Some(args), &mut warnings)
				.to_string();

			for e in warnings {
				warn!("{e}");
			}
			out
		} else {
			error!("Lang {lang_key} lacks key {key}");
			key.to_string()
		}
	} else {
		error!("Language {lang_key} not found");
		key.to_string()
	}
}
