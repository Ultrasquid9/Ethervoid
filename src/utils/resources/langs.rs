use tracing::error;

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

/// Gets the image at the provided key
pub fn access_lang(key: &str) -> &str {
	let lang_key = &access_config().lang;

	if let Some(lang) = get_resource_ref(&LANGS, lang_key) {
		if let Some(lang) = lang.get(key) {
			lang
		} else {
			error!("Lang {lang_key} lacks key {key}");
			key
		}
	} else {
		error!("Language {lang_key} not found");
		key
	}
}
