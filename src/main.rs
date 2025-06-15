use crate::utils::resources::{
	config::{read_config, update_config},
	create_resources,
	save::save,
};

use self::prelude::*;
use gameplay::gameplay;

use menu::{init_ui, main::menu};
use utils::{error::EvoidResult, logger::init_log};

mod cores;
mod data;
mod gameplay;
mod menu;
mod utils;

/// Used to determine what state the game is in.
pub enum State {
	/// The main-menu
	Menu,
	/// In-game
	Gameplay,
	/// Exiting the gamej
	Quit,
}

#[macroquad::main("Ethervoid")]
async fn main() -> EvoidResult<()> {
	init_log().await?;
	init_ui().await?;

	let mut state = State::Menu;

	loop {
		// Locates and creates all the resources in the game (textures, maps, etc.)
		create_resources();
		// Updates the config
		update_config(read_config());
		// Saves the game
		// TODO: Save game explicitly
		save();

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

	pub use rustc_hash::FxHashMap;

	pub use tracing::{debug, error, info, trace, warn};
}
