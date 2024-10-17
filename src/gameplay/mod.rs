use builders::mapbuilder::get_mapbuilders;
use enemy::Enemy;
use player::Player;
use macroquad::prelude::*;

use crate::{input::get_keycode, State};

mod player;
mod enemy;
mod builders;

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pub pos: Vec2,
	pub health: isize
}

pub async fn gameplay() -> State {
	let mut player = Player::new(); // Creates a player
	let mut enemies = Vec::new(); // Creates a list of enemies
	
	let maps = get_mapbuilders(); // Creates a list of MapBuilders
	let current_map = String::from("Test"); // Stores the map the player is currently in

	for i in maps.get(&current_map).unwrap().enemies.clone() {
		enemies.push(Enemy::from_builder(i.1, i.0))
	}
	
	get_mapbuilders();
	
	loop {
		clear_background(RED); // Draws the background

		// Creates a camera targetting the player
        set_camera(&Camera2D {
			zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
            target: player.stats.pos,
            ..Default::default()
        });

		// Updates the player and all enemies
		player.update();

		if enemies.len() > 0 {
			update_enemies(&mut player, &mut enemies);

			let enemies_to_kill = enemies_to_kill(&enemies);
			enemies.retain(|_| *enemies_to_kill.iter().next().unwrap());
		}

		// Drawing the Player and enemies
        draw_circle(player.stats.pos.x, player.stats.pos.y, 15.0, YELLOW); // Player
		if enemies.len() > 0 {
			for i in &enemies {
				draw_circle(i.stats.pos.x, i.stats.pos.y, 15.0, GREEN); // Enemies
			}
		}

		set_default_camera();
 
		// Drawing a temporary UI
		draw_text(&format!("{}", player.stats.health), 32.0, 64.0, camera_scale() / 10., BLACK);

		// Quits the game
		if is_key_down(get_keycode(&player.config, "Quit")) {
			println!("Quitting the game");
			return State::Quit;
		}

		next_frame().await;
	}
}

/// Gets the scale that the camera should be rendered at
fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
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
