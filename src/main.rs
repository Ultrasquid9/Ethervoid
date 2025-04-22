use self::prelude::*;
use gameplay::gameplay;

use menu::{init_ui, main::menu};
use utils::{error::EvoidResult, logger::init_log};

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
async fn main() -> EvoidResult<()> {
	init_log().await?;
	init_ui().await?;

	let mut state = State::Menu;

	loop {
		state = match state {
			State::Menu => menu().await,
			State::Gameplay => gameplay().await,
			State::Quit => return Ok(()),
		};

		next_frame().await;
	}
}

pub mod prelude {
	pub use macroquad::prelude::*;
	pub use stecs::prelude::*;

	pub use ahash::HashMap;

	pub use tracing::{debug, error, info, trace, warn};
}
