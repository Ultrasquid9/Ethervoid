use std::fs;

use macroquad::input::{
	is_key_down, 
	is_key_pressed, 
	is_mouse_button_down, 
	is_mouse_button_pressed, 
	KeyCode, 
	MouseButton
};

use serde::{
	Deserialize, 
	Serialize
};

const DEFAULT_CONFIG: &str = "
(
	keymap: (
		up: KeyCode(W),
		left: KeyCode(A),
		down: KeyCode(S),
		right: KeyCode(D),
		dash: KeyCode(LeftShift),

		sword: MouseButton(Left),
		gun: MouseButton(Right),
		change_sword: KeyCode(R),
		change_gun: KeyCode(F),

		quit: KeyCode(Escape)
	)
)
";

/// The config for the game
#[derive(Serialize, Deserialize)]
pub struct Config {
	pub keymap: KeyMap
}

/// The different possible inputs for the player
#[derive(Serialize, Deserialize)]
pub struct KeyMap {
	pub up: Key,
	pub down: Key,
	pub left: Key,
	pub right: Key,
    pub dash: Key,

	pub sword: Key,
	pub gun: Key,
	pub change_sword: Key,
	pub change_gun: Key,

	pub quit: Key
}

/// Contains both keyboard and mouse buttons
#[derive(Serialize, Deserialize)]
pub enum Key{
	#[serde(with = "KeyCodeSerialize")]
	KeyCode(KeyCode),

	#[serde(with = "MouseButtonSerialize")]
	MouseButton(MouseButton)
}

impl Config {
	/// Reads the config file 
	pub fn read(dir: &str) -> Self {
		match ron::from_str(
			&match fs::read_to_string(dir) {
				Ok(str) => str,
				Err(_) => DEFAULT_CONFIG.to_string()
			}
		) {
			Ok(config) => config,
			Err(_) => ron::from_str(DEFAULT_CONFIG).unwrap()
		}
	}
}

impl Key {
	/// Checks if the key is down
	pub fn is_down(&self) -> bool {
		match self {
			Self::KeyCode(button) => {
				if is_key_down(*button) {return true}
			},
			Self::MouseButton(button) => {
				if is_mouse_button_down(*button) {return true}
			}
		}
		false
	}

	/// Checks if the key is pressed
	pub fn is_pressed(&self) -> bool {
		match self {
			Self::KeyCode(button) => {
				if is_key_pressed(*button) {return true}
			},
			Self::MouseButton(button) => {
				if is_mouse_button_pressed(*button) {return true}
			}
		}
		false
	}
}

// Gaze upon this in horror, 
// and see what orphan rules
// make one do.

#[derive(Serialize, Deserialize)]
#[serde(remote = "MouseButton")]
enum MouseButtonSerialize {
    Left = 0,
    Middle = 1,
    Right = 2,
    Unknown = 255,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "KeyCode")]
enum KeyCodeSerialize {
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Key0 = 48,
    Key1 = 49,
    Key2 = 50,
    Key3 = 51,
    Key4 = 52,
    Key5 = 53,
    Key6 = 54,
    Key7 = 55,
    Key8 = 56,
    Key9 = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    GraveAccent = 96,
    World1 = 256,
    World2 = 257,
    Escape = 65_307,
    Enter = 65_293,
    Tab = 65_289,
    Backspace = 65_288,
    Insert = 65_379,
    Delete = 65_535,
    Right = 65_363,
    Left = 65_361,
    Down = 65_364,
    Up = 65_362,
    PageUp = 65_365,
    PageDown = 65_366,
    Home = 65_360,
    End = 65_367,
    CapsLock = 65_509,
    ScrollLock = 65_300,
    NumLock = 65_407,
    PrintScreen = 64_797,
    Pause = 65_299,
    F1 = 65_470,
    F2 = 65_471,
    F3 = 65_472,
    F4 = 65_473,
    F5 = 65_474,
    F6 = 65_475,
    F7 = 65_476,
    F8 = 65_477,
    F9 = 65_478,
    F10 = 65_479,
    F11 = 65_480,
    F12 = 65_481,
    F13 = 65_482,
    F14 = 65_483,
    F15 = 65_484,
    F16 = 65_485,
    F17 = 65_486,
    F18 = 65_487,
    F19 = 65_488,
    F20 = 65_489,
    F21 = 65_490,
    F22 = 65_491,
    F23 = 65_492,
    F24 = 65_493,
    F25 = 65_494,
    Kp0 = 65_456,
    Kp1 = 65_457,
    Kp2 = 65_458,
    Kp3 = 65_459,
    Kp4 = 65_460,
    Kp5 = 65_461,
    Kp6 = 65_462,
    Kp7 = 65_463,
    Kp8 = 65_464,
    Kp9 = 65_465,
    KpDecimal = 65_454,
    KpDivide = 65_455,
    KpMultiply = 65_450,
    KpSubtract = 65_453,
    KpAdd = 65_451,
    KpEnter = 65_421,
    KpEqual = 65_469,
    LeftShift = 65_505,
    LeftControl = 65_507,
    LeftAlt = 65_513,
    LeftSuper = 65_515,
    RightShift = 65_506,
    RightControl = 65_508,
    RightAlt = 65_514,
    RightSuper = 65_516,
    Menu = 65_383,
    Unknown = 511,
}
