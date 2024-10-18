use builders::mapbuilder::get_mapbuilders;
use draw::draw;
use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

use crate::{input::get_keycode, State};

mod player;
mod enemy;
mod builders;
mod draw;

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pub pos: Vec2,
	pub health: isize
}

pub async fn gameplay() -> State {
	// The player and enemies themselves
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	
	// The maps
	let maps = get_mapbuilders(); // Creates a list of MapBuilders
	let current_map = String::from("Test"); // Stores the map the player is currently in

	// Populating the enemies with data from the maps
	for i in maps.get(&current_map).unwrap().enemies.clone() {
		enemies.push(Enemy::from_builder(i.1, i.0))
	}

	loop {
		// Updates the player and all enemies
		player.update();

		if enemies.len() > 0 {
			update_enemies(&mut player, &mut enemies);

			let enemies_to_kill = enemies_to_kill(&enemies);
			enemies.retain(|_| *enemies_to_kill.iter().next().unwrap());
		}

		// Draws the player and enemies
		draw(&player, &enemies);

		// Quits the game
		if is_key_down(get_keycode(&player.config, "Quit")) {
			println!("Quitting the game");
			return State::Quit;
		}

		next_frame().await;
	}
}

fn update_enemies(player: &mut Player, enemies: &mut Vec<Enemy>) {
	for i in enemies {
		i.update(player);

		if is_key_down(get_keycode(&player.config, "Attack")) {
			if i.stats.pos.distance(player.stats.pos) < 64.0 {
				i.damage(1);
			}
		}
	}
}

fn enemies_to_kill(enemies: &Vec<Enemy>) -> Vec<bool> {
	let mut enemies_to_kill: Vec<bool> = Vec::new();

	for i in enemies {
		enemies_to_kill.push(!i.should_kill());
	}

	return enemies_to_kill;
}
