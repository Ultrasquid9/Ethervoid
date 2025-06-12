use std::ops::{Deref, DerefMut};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::cores::{DIR_SPLIT, Readable, gen_name, get_files};

#[derive(Serialize, Deserialize)]
pub struct Lang(HashMap<String, String>);

impl Readable for Lang {}

impl Deref for Lang {
	type Target = HashMap<String, String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Lang {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub fn get_langs() -> HashMap<String, Lang> {
	let mut langs: HashMap<String, Lang> = HashMap::new();

	for dir in get_files("lang") {
		let display_name = gen_name(&dir);
		let lang_name = gen_lang_name(&dir);

		let lang = match Lang::read(&dir) {
			Ok(ok) => {
				info!("Lang {display_name} loaded!");
				ok
			}
			Err(e) => {
				warn!("Lang {display_name} failed to load: {e}");
				continue;
			}
		};

		if let Some(existing) = langs.get_mut(&lang_name) {
			existing.extend(lang.0.into_iter());
		} else {
			langs.insert(lang_name, lang);
		}
	}

	langs
}

fn gen_lang_name(dir: &str) -> String {
	let mut split: Vec<&str> = dir.split(DIR_SPLIT).collect();

	split.pop();
	split.pop().unwrap_or_default().into()
}
