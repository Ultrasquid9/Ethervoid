use macroquad::input::KeyCode;
use serde_json::Value;
use std::fs;

pub fn get_keycode(key: &str) -> KeyCode {
	let try_input = serde_json::from_str(&fs::read_to_string("./config.json").expect("Config does not exist!"));
	let input: Value = match try_input {
		Ok(input) => input,
		Err(error) => panic!("Error reading config: {}", error)
	};
	
	match input[&key].as_str() {
		Some("Up") => return KeyCode::Up,
		Some("Down") => return KeyCode::Down,
		Some("Left") => return KeyCode::Left,
		Some("Right") => return KeyCode::Right,
		_ => panic!("Bad keycode: {} is not a valid value for {}", input[&key], key)
	}
}
