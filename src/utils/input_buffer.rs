use ahash::HashMap;
use struct_iterable::Iterable;

use super::{
	config::{
		Config, 
		Key
	}, 
	get_delta_time
};

pub struct InputBuffer {
	buffer: HashMap<Key, (f32, u32)>,
	keys_pressed: u32
}

impl InputBuffer {
	pub fn new() -> Self {
		Self {
			buffer: HashMap::default(),
			keys_pressed: 0
		}
	}

	pub fn handle_input(&mut self, config: &Config) {
		for (_, (lifetime, _)) in self.buffer.iter_mut() {
			*lifetime -= get_delta_time();
		}

		self.buffer.retain(|_, (lifetime, _)| *lifetime > 0.);
		if self.buffer.is_empty() { self.keys_pressed = 0 }

		for (_, key) in config.keymap.iter() {
			let key = key.downcast_ref::<Key>().unwrap();

			if key.is_down() {
				let key_press_time = {
					if !self.buffer.contains_key(key) {
						self.keys_pressed += 1;
						self.keys_pressed
					} else {
						self.buffer.get(key).unwrap().1
					}
				};

				self.buffer.insert(*key, (3., key_press_time));
			}
		}
	}

	pub fn was_pressed(&self, key: &Key) -> bool {
		self.buffer.contains_key(key) 
	}

	pub fn most_recent<'b>(&'b self, key1: &'b Key, key2: &'b Key) -> &'b Key {
		if !self.buffer.contains_key(key1) { return key2 }
		if !self.buffer.contains_key(key2) { return key1 }
		
		if self.buffer.get(&key1).unwrap().1 > self.buffer.get(&key2).unwrap().1 {
			key1
		} else {
			key2
		}
	}
}
