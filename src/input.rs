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

		_ => panic!("Bad keycode: {} is not a valid value for {}", input[&key], key)
	}
}
