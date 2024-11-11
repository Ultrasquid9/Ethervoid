use cores::map::{get_maps, Map};
use combat::{try_parry, Attack};
use draw::{clean_textures, create_textures, draw};
use enemy::Enemy;
use entity::MovableObj;
use npc::NPC;
use player::Player;
use macroquad::prelude::*;
use stecs::prelude::*;

use crate::State;

mod combat;
mod cores;
mod doors;
mod draw;
mod enemy;
mod entity;
mod npc;
mod player;

#[derive(SplitFields)]
pub struct EnemyArch{
	io: Enemy	
}

#[derive(SplitFields)]
pub struct NPCArch{
	io: NPC	
}

#[derive(SplitFields)]
pub struct AttackArch{
	io: Attack	
}

pub struct World {
	enemies: StructOf<Vec<EnemyArch>>,
	npcs: StructOf<Vec<NPCArch>>,
	attacks: StructOf<Vec<AttackArch>>
}

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
		attacks: Default::default()
	};

	// The player
	let mut player = Player::new(); // Creates a player
	
	// The maps
	let maps = get_maps(); // Creates a list of Maps
	let mut current_map = String::from("default:test"); // Stores the map the player is currently in

	// Populating the enemies with data from the maps
	populate(&mut world, maps.get(&current_map).unwrap());

	loop {
		// Updates the player
		player.update(
			&mut camera, 
			
			&mut world,
			
			&mut current_map,
			&maps
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
		let mut to_remove: Vec<usize> = Vec::new();
		for (index, attack) in world.attacks.iter_mut() {
			attack.io.update(&mut world.enemies.io, &mut player, &maps.get(&current_map).unwrap());

			if attack.io.should_rm() {
				to_remove.push(index);
			}
		}
		try_parry(&mut world);
		for i in to_remove {
			world.attacks.remove(i);
		}

		// Updates enemies
		let mut to_remove: Vec<usize> = Vec::new();
		for (index, enemy) in world.enemies.iter_mut() {
			enemy.io.update(&mut world.attacks, &mut player, &maps.get(&current_map).unwrap());

			if enemy.io.stats.should_kill() {
				to_remove.push(index);
			}
		}
		for i in to_remove {
			world.enemies.remove(i);
		}

		// Updates NPCs
		// WIP
		for (_, npc) in world.npcs.iter_mut() {
			npc.io.update(&maps.get(&current_map).unwrap());
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
			&maps.get(&current_map).unwrap()
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

pub fn populate(world: &mut World, map: &Map) {
	// Removing all the old content of the world 
	// This is a lot more wordy than I would like,
	// But oh well. 

	// Enemies
	let mut enemy_ids: Vec<usize> = Vec::new();
	for i in world.enemies.ids() {
		enemy_ids.push(i);
	}
	for i in enemy_ids {
		world.enemies.remove(i);
	}

	// NPCs
	let mut npc_ids: Vec<usize> = Vec::new();
	for i in world.npcs.ids() {
		npc_ids.push(i);
	}
	for i in npc_ids {
		world.npcs.remove(i);
	}

	// Attacks
	let mut attack_ids: Vec<usize> = Vec::new();
	for i in world.attacks.ids() {
		attack_ids.push(i);
	}
	for i in attack_ids {
		world.attacks.remove(i);
	}

	// Adding the new enemies
	for i in map.enemies.clone() {
		world.enemies.insert(EnemyArch { io: Enemy::new(i.1, i.0.clone())});
	}

	// Adding the new NPCs
	for i in map.npcs.clone() {
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
