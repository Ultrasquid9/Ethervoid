use crate::{
	data::save::Save,
	utils::resources::{Global, GlobalAccess, GlobalAccessMut, global},
};

// TODO: Make more configurable.
const SAVE_DIR: &str = "./save.evoid";

static SAVE: Global<Save> = global!(Save::read(SAVE_DIR));

/// Gets the current save
#[allow(unused)] // TODO: Use
pub fn access_save() -> GlobalAccess<Save> {
	SAVE.read()
}

/// Gets the current save mutably
pub fn access_save_mut() -> GlobalAccessMut<Save> {
	SAVE.write()
}

/// Saves the game
pub fn save() {
	SAVE.read().save(SAVE_DIR);
}
