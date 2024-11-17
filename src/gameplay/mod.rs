use cores::map::get_maps;
use combat::try_parry;
use draw::draw;
use enemy::Enemy;
use entity::MovableObj;
use npc::NPC;
use player::Player;
use macroquad::prelude::*;
use stecs::prelude::Archetype;

use ecs::{
	AttackArch, 
	EnemyArch, 
	NPCArch, 
	PlayerArch, 
	World
};

use crate::{
	utils::{
		resources::{
			clean_resources, 
			create_resources
		},
		get_delta_time
	}, 
	State
};

pub mod cores;
pub mod draw;
mod combat;
mod doors;
mod ecs;
mod enemy;
mod entity;
mod npc;
mod player;

/// The gameplay loop of the game
pub async fn gameplay() -> State {
	// The camera
	let mut camera = Vec2::new(0., 0.);

	// Resources
	// NOTE: This function populates a static HashMap. Ensure you call the `clean_resources()` functions when quitting the game. 
	create_resources();

	// The world 
	// Contains Enemies, Attacks, ETC.
	let mut world = World {
		player: Default::default(),
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
	let player = world.player.insert(PlayerArch {io: Player::new()}); // Creates a player

	// Populating the enemies with data from the maps
	populate(&mut world);

	loop {
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();
			draw(&mut camera, &world, &world.get_current_map()).await;

			next_frame().await;
			continue;
		}

		// Updating the player
		 
		// Stores the old map, in case it changes
		let old_map = world.current_map.clone();

		world.player.io[player].update(
			&mut camera, 
			
			&world.maps,
			&mut world.current_map
		);

		// If the current map has changed, repopulate the world
		if world.current_map != old_map {
			populate(&mut world);
		}

		// Attacking
		if world.player.io[player].config.keymap.sword.is_down() && world.player.io[player].swords[0].cooldown == 0 {
			world.player.io[player].swords[0].cooldown = 16;
			world.attacks.insert( AttackArch { io: world.player.io[player].attack_sword() });
		}
		if world.player.io[player].config.keymap.gun.is_down() && world.player.io[player].guns[0].cooldown == 0 {
			world.player.io[player].guns[0].cooldown = 16;
			world.attacks.insert( AttackArch { io: world.player.io[player].attack_gun() });
		}

		// Updates attacks
		for (_, attack) in world.attacks.iter_mut() {
			attack.io.update(&mut world.enemies.io, &mut world.player.io[player], world.maps.get(&world.current_map).unwrap());
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
			enemy.io.update(&mut world.attacks, &mut world.player.io[player], world.maps.get(&world.current_map).unwrap());
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
			npc.io.update(world.maps.get(&world.current_map).unwrap(), &world.attacks);
		}

		// Updates the camera
		// TODO: Attempt to replace with .lerp()
		camera = camera.move_towards(
			world.player.io[player].stats.get_pos(), 
			camera.distance(world.player.io[player].stats.get_pos()) / 6.
		);

		// Draws the player and enemies
		draw(
			&mut camera, 
			&world, 
			&world.get_current_map()
		).await;

		// Quits the game
		if world.player.io[player].config.keymap.quit.is_pressed() {
			clean_resources();

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
