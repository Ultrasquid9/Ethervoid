use draw::draw;
use ecs::{behavior::handle_behavior, World};
use enemy::Enemy;
use macroquad::window::next_frame;
use npc::Npc;
use player::Player;
use stecs::prelude::Archetype;

use crate::{utils::resources::{create_resources, maps::access_map}, State};

pub mod combat;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub async fn gameplay() -> State {
	unsafe { create_resources(); } // TODO: Clean resources (irrelevant until main menu is reimplemented)

	let current_map = String::from("default:test");

	let mut world = World {
		player: Default::default(),
		enemies: Default::default(),
		npcs: Default::default(),
		attacks: Default::default()
	};

	for (enemy, pos) in access_map(&current_map).enemies.iter() {
		let _ = world.enemies.insert(Enemy::from_type(enemy, pos));
	}

	for (npc, pos) in access_map(&current_map).npcs.iter() {
		let _ = world.npcs.insert(Npc::from_type(npc, pos));
	}

	world.player.insert(Player::new());

	loop {
		draw(&world).await;

		handle_behavior(&mut world);

		next_frame().await
	}
}