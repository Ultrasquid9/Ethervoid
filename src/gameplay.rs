use draw::draw;
use ecs::World;
use macroquad::window::next_frame;
use player::Player;
use stecs::prelude::Archetype;

use crate::State;

pub mod combat;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod player;

pub async fn gameplay() -> State {
	let mut world = World {
		player: Default::default(),
		enemies: Default::default()
	};

	world.player.insert(Player::new());

	loop {
		draw(&world).await;

		next_frame().await
	}
}