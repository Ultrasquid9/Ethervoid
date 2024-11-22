use draw::draw;
use ecs::{behavior::handle_behavior, World};
use macroquad::window::next_frame;
use player::Player;
use stecs::prelude::Archetype;

use crate::State;

pub mod combat;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub async fn gameplay() -> State {
	let mut world = World {
		player: Default::default(),
		enemies: Default::default(),
		attacks: Default::default()
	};

	world.player.insert(Player::new());

	loop {
		draw(&world).await;

		handle_behavior(&mut world);

		next_frame().await
	}
}