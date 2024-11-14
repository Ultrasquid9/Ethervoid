use cores::map::get_maps;
use combat::try_parry;
use draw::{clean_textures, create_textures, draw};
use ecs::{AttackArch, EnemyArch, NPCArch, World};
use enemy::Enemy;
use entity::MovableObj;
use npc::NPC;
use player::Player;
use macroquad::prelude::*;
use stecs::prelude::Archetype;

use crate::State;

mod combat;
mod cores;
mod doors;
mod draw;
mod ecs;
mod enemy;
mod entity;
mod npc;
mod player;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The camera
	let mut camera = Vec2::new(0., 0.);

	// Textures
	// NOTE: populates a static HashMap. Ensure you call the `clean_textures` function when quitting the game. 
	create_textures();

	// The world 
	// Contains Enemies, Attacks, ETC.
	let mut world = World {
		enemies: Default::default(),
		npcs: Default::default(),
		attacks: Default::default(),

		// Resources
		hitstop: 0.,

		// Maps
		maps: get_maps(),
		current_map: String::from("default:test") 
	};

	// The player
	let mut player = Player::new(); // Creates a player

	// Populating the enemies with data from the maps
	populate(&mut world);

	loop {
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();
			draw(&mut camera, &player, &world, &world.get_current_map()).await;

			next_frame().await;
			continue;
		}

		// Updates the player
		player.update(
			&mut camera, 
			
			&mut world
		);

		// Attacking
		if player.config.keymap.sword.is_down() && player.swords[0].cooldown == 0 {
			player.swords[0].cooldown = 16;
			world.attacks.insert( AttackArch { io: player.attack_sword() });
		}
		if player.config.keymap.gun.is_down() && player.guns[0].cooldown == 0 {
			player.guns[0].cooldown = 16;
			world.attacks.insert( AttackArch { io: player.attack_gun() });
		}

		// Updates attacks
		for (_, attack) in world.attacks.iter_mut() {
			attack.io.update(&mut world.enemies.io, &mut player, world.maps.get(&world.current_map).unwrap());
		}
		try_parry(&mut world);
		// Removing old attacks
		let mut to_remove: usize = 0;
		while (|| {
			for (index, attack) in world.attacks.iter() {
				if attack.io.should_rm() {
					to_remove = index;
					return true
				}
			}
			return false
		})() {
			world.attacks.remove(to_remove);
		}

		// Updates enemies
		for (_, enemy) in world.enemies.iter_mut() {
			enemy.io.update(&mut world.attacks, &mut player, world.maps.get(&world.current_map).unwrap());
		}
		// Removing dead enemies
		let mut to_remove: usize = 0;
		while (|| {
			for (index, enemy) in world.enemies.iter() {
				if enemy.io.stats.should_kill() {
					to_remove = index;
					return true
				}
			}
			return false
		})() {
			world.enemies.remove(to_remove);
		}

		// Updates NPCs
		// WIP
		for (_, npc) in world.npcs.iter_mut() {
			npc.io.update(world.maps.get(&world.current_map).unwrap());
		}

		// Updates the camera
		// TODO: Attempt to replace with .lerp()
		camera = camera.move_towards(
			player.stats.get_pos(), 
			camera.distance(player.stats.get_pos()) / 6.
		);

		// Draws the player and enemies
		draw(
			&mut camera, 
			&player, 
			&world, 
			&world.get_current_map()
		).await;

		// Quits the game
		if player.config.keymap.quit.is_pressed() {
			clean_textures();

			println!("Returning to the main menu");
			return State::Menu;
		}

		next_frame().await;
	}
}

pub fn populate(world: &mut World) {
	// Removing all the old content of the world 

	// Enemies
	while world.enemies.io.len() > 0 {
		world.enemies.remove(0);
	}
	// NPCs
	while world.npcs.io.len() > 0 {
		world.npcs.remove(0);
	}
	// Attacks
	while world.attacks.io.len() > 0 {
		world.attacks.remove(0);
	}

	// Adding the new content

	// Enemies
	for i in world.get_current_map().enemies.clone() {
		world.enemies.insert(EnemyArch { io: Enemy::new(i.1, i.0.clone())});
	}
	// NPCs
	for i in world.get_current_map().npcs.clone() {
		world.npcs.insert(NPCArch { io: NPC::new(i.0, i.1)});
	}
}

/// Converts inputted Vec2 into a tuple of f32
pub fn vec2_to_tuple(vec: &Vec2) -> (f32, f32) {
	return (vec.x, vec.y);
}

/// Converts the inputted tuple of f32 into a Vec2
pub fn tuple_to_vec2(tup: (f32, f32)) -> Vec2 {
	return Vec2::new(tup.0, tup.1);
}

/// Gets the current position of the mouse
pub fn get_mouse_pos() -> Vec2 {
	tuple_to_vec2(mouse_position()) - Vec2::new(screen_width() / 2., screen_height() / 2.)
}

/// Gets the delta time
pub fn get_delta_time() -> f32 {
	get_frame_time() * 100. * (2./3.)
}
