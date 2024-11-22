use audio::{clean_audio, create_audio};
use textures::{clean_textures, create_textures};

mod audio;
mod textures;

// This module contains globally available resources
// Everyone always says "don't do this" so fuck you I did

/// Populates global resources
/// NOTE: Please ensure you call `clean_resources()` when quitting the game
pub unsafe fn create_resources() {
	create_audio();
	create_textures();
}

/// Cleans the global resources
pub fn clean_resources() {
	clean_audio();
	clean_textures();
}
