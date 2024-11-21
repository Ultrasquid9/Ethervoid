use ecs::World;
use macroquad::window::next_frame;
use player::Player;
use stecs::prelude::Archetype;

use crate::State;

pub mod ecs;
pub mod player;

pub async fn gameplay() -> State {
	let mut world = World {
		player: Default::default()
	};

	world.player.insert(Player::new());

	loop {
		next_frame().await
	}
}