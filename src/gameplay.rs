use draw::draw;
use ecs::{behavior::handle_behavior, World};
use enemy::Enemy;
use macroquad::window::next_frame;
use player::Player;
use stecs::prelude::Archetype;

use crate::{cores::map::get_maps, State};

pub mod combat;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub async fn gameplay() -> State {
	let maps = get_maps();
	let current_map = String::from("default:test");

	let mut world = World {
		player: Default::default(),
		enemies: Default::default(),
		attacks: Default::default()
	};

	for (enemy, pos) in maps.get(&current_map).unwrap().enemies.iter() {
		let _ = world.enemies.insert(Enemy::from_type(enemy, pos));
	}

	world.player.insert(Player::new());

	loop {
		draw(&world).await;

		handle_behavior(&mut world);

		next_frame().await
	}
}