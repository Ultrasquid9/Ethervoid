use self::prelude::*;
use gameplay::gameplay;

use menu::{menu, ui::init_ui};
use utils::init_log;

mod cores;
mod gameplay;
mod menu;
mod utils;

/// Used to determine what state the game is in.
pub enum State {
	Menu,     // The main-menu
	Gameplay, // In-game
	Quit,     // Exiting the gamej
}

#[macroquad::main("Ethervoid")]
async fn main() {
	init_log();
	init_ui();

	let mut state = State::Menu;

	loop {
		state = match state {
			State::Menu => menu().await,
			State::Gameplay => gameplay().await,
			State::Quit => return,
		};

		next_frame().await
	}
}

pub mod prelude {
	pub use macroquad::prelude::*;
	pub use rayon::prelude::*;
	pub use stecs::prelude::*;

	pub use ahash::HashMap;

	pub use log::{debug, error, info, trace, warn};
}
