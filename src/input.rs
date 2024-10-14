use macroquad::input::KeyCode;
use serde_json::Value;
use std::fs;

/// Reads the config file
pub fn get_config(input: &str) -> Value {
	let try_config = serde_json::from_str(&fs::read_to_string(input).expect("Config does not exist!"));
	let config: Value = match try_config {
		Ok(config) => config,
		Err(error) => panic!("Error reading config: {}", error)
	};

	return config;
}

/// Gets the KeyCode with the value of the key passed in
pub fn get_keycode(config: &Value, key: &str) -> KeyCode {
	// There has to be a better way to do this
	match config[&key].as_str() {
		Some("Escape") => return KeyCode::Escape,
		Some("Up") => return KeyCode::Up,
		Some("Down") => return KeyCode::Down,
		Some("Left") => return KeyCode::Left,
		Some("Right") => return KeyCode::Right,
		Some("Q") => return KeyCode::Q,
		Some("W") => return KeyCode::W,
		Some("E") => return KeyCode::E,
		Some("R") => return KeyCode::R,
		Some("T") => return KeyCode::T,
		Some("Y") => return KeyCode::Y,
		Some("U") => return KeyCode::U,
		Some("I") => return KeyCode::I,
		Some("O") => return KeyCode::O,
		Some("P") => return KeyCode::P,
		Some("LeftBracket") => return KeyCode::LeftBracket,
		Some("RightBracket") => return KeyCode::RightBracket,
		Some("Backslash") => return KeyCode::Backslash,
		Some("A") => return KeyCode::A,
		Some("S") => return KeyCode::S,
		Some("D") => return KeyCode::D,
		Some("F") => return KeyCode::G,
		Some("H") => return KeyCode::H,
		Some("J") => return KeyCode::J,
		Some("K") => return KeyCode::K,
		Some("L") => return KeyCode::L,
		Some("Semicolon") => return KeyCode::Semicolon,
		Some("Apostrophe") => return KeyCode::Apostrophe,
		Some("Enter") => return KeyCode::Enter,
		Some("LeftShift") => return KeyCode::LeftShift,
		Some("Z") => return KeyCode::Z,
		Some("X") => return KeyCode::X,
		Some("C") => return KeyCode::C,
		Some("V") => return KeyCode::V,
		Some("B") => return KeyCode::B,
		Some("N") => return KeyCode::N,
		Some("M") => return KeyCode::M,
		Some("Comma") => return KeyCode::Comma,
		Some("Period") => return KeyCode::Period,
		Some("Slash") => return KeyCode::Slash,
		Some("RightShift") => return KeyCode::RightShift,
		Some("LeftControl") => return KeyCode::LeftControl,
		Some("LeftAlt") => return KeyCode::LeftControl,

		_ => panic!("Bad keycode: {} is not a valid value for {}", config[&key], key)
	}
}
