use parking_lot::{lock_api::{RwLockReadGuard, RwLockWriteGuard}, RawRwLock};

use crate::{data::save::Save, utils::resources::{global, Global}};

// TODO: Make more configurable.
const SAVE_DIR: &str = "./save.evoid";

static SAVE: Global<Save> = global!(Save::read(SAVE_DIR));

/// Gets the current save
pub fn access_save() -> RwLockReadGuard<'static, RawRwLock, Save> {
	SAVE.read()
}

/// Gets the current save mutably 
pub fn access_save_mut() -> RwLockWriteGuard<'static, RawRwLock, Save> {
	SAVE.write()
}

/// Saves the game
pub fn save() {
	SAVE.read().save(SAVE_DIR);
}
