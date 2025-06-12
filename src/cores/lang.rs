use std::error::Error;

use fluent::{FluentResource, concurrent::FluentBundle};
use hashbrown::HashMap;
use tracing::{info, warn};
use unic_langid::LanguageIdentifier;

use crate::{
	cores::{DIR_SPLIT, gen_name, get_files},
	utils::error::EvoidResult,
};

pub type Lang = FluentBundle<FluentResource>;

pub fn get_langs() -> HashMap<String, Lang> {
	let mut langs: HashMap<String, Lang> = HashMap::new();

	for dir in get_files("lang") {
		let lang_name = gen_name(&dir);
		let lang_id = match gen_lang_id(&dir) {
			Ok(ok) => ok,
			Err(e) => {
				warn!("Lang {lang_name} has invalid name: {e}");
				continue;
			}
		};

		let lang = match read_fluent_file(&dir) {
			Ok(ok) => {
				info!("Lang {lang_name} loaded!");
				ok
			}
			Err(e) => {
				warn!("Lang {lang_name} failed to load: {e}");
				continue;
			}
		};

		let lang_id_str = lang_id.language.as_str().to_string();

		if let Some(existing) = langs.get_mut(&lang_id_str) {
			log_if_err(existing.add_resource(lang));
		} else {
			let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
			log_if_err(bundle.add_resource(lang));
			langs.insert(lang_id_str, bundle);
		}
	}

	langs
}

fn gen_lang_id(dir: &str) -> EvoidResult<LanguageIdentifier> {
	let mut split: Vec<&str> = dir.split(DIR_SPLIT).collect();

	split.pop();
	Ok(split.pop().unwrap_or_default().parse()?)
}

fn read_fluent_file(dir: &str) -> EvoidResult<FluentResource> {
	let file = std::fs::read_to_string(dir)?;
	match FluentResource::try_new(file) {
		Ok(ok) => Ok(ok),
		Err((ok, err)) => {
			for e in err {
				warn!("{e}");
			}

			Ok(ok)
		}
	}
}

fn log_if_err(maybe_err: Result<(), Vec<impl Error>>) {
	_ = maybe_err.map_err(|err| {
		for e in err {
			warn!("{e}");
		}
	});
}
