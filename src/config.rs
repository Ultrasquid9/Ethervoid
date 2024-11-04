use macroquad::input::{is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, KeyCode, MouseButton};
use serde::{Deserialize, Serialize};
use std::{fs, ops::Deref};

const MOUSE_KEYS: [&str; 3] = ["Left Click", "Right Click", "Middle Click"];

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub keymap: KeyMap
}

#[derive(Serialize, Deserialize)]
pub struct KeyMap {
	pub up: Key,
	pub down: Key,
	pub left: Key,
	pub right: Key,

	pub sword: Key,
	pub gun: Key,
	pub change_sword: Key,
	pub change_gun: Key,

	pub quit: Key
}

#[derive(Serialize, Deserialize)]
pub struct Key (String);

impl Config {
	/// Reads the config file 
	pub fn read(dir: &str) -> Self {
		return ron::from_str(&fs::read_to_string(dir).unwrap()).unwrap();
	}
}

impl Key {
	/// Checks if the key is down
	pub fn is_down(&self) -> bool {
		if MOUSE_KEYS.contains(&&*self.as_str()) {
			if is_mouse_button_down(get_mousebutton(Some(self))) {
				return true
			}
		} else {
			if is_key_down(get_keycode(Some(self))) {
				return true
			}
		}
		return false
	}

	/// Checks if the key is pressed
	pub fn is_pressed(&self) -> bool {
		if MOUSE_KEYS.contains(&&*self.as_str()) {
			if is_mouse_button_pressed(get_mousebutton(Some(self))) {
				return true
			}
		} else {
			if is_key_pressed(get_keycode(Some(self))) {
				return true
			}
		}
		return false
	}
}

impl Deref for Key {
	type Target = String;

	fn deref(&self) -> &Self::Target { &self.0 }
}

/// Gets the KeyCode with the value of the key passed in
fn get_keycode(key: Option<&str>) -> KeyCode {
	// There has to be a better way to do this
	match key {
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

		_ => panic!("Bad keycode: {} is not a valid value", key.unwrap())
	}
}

/// Gets the MouseButton of the key passed to it 
fn get_mousebutton(key: Option<&str>) -> MouseButton {
	match key {
		Some("Left Click") => return MouseButton::Left,
		Some("Right Click") => return MouseButton::Right,
		Some("Middle Click") => return MouseButton::Middle,

		_ => panic!("Bad keycode: {} is not a valid value", key.unwrap())
	}
}
