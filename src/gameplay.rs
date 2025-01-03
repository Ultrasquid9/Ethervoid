use draw::{draw, render::darken_screen};
use stecs::prelude::*;
use macroquad::prelude::*;

use crate::{
	utils::{
		resources::{
			clean_resources, 
			create_resources
		}, 
		get_delta_time,  
		update_delta_time,
		config::Config,
	}, 
	State
};

use combat::{
	handle_combat, 
	AttackType, 
	Owner
};

use ecs::{
	behavior::handle_behavior, 
	World
};

use player::{
	swap_weapons, 
	Player
};

pub mod combat;
pub mod doors;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub async fn gameplay() -> State {
	unsafe { create_resources(); } // All the resources in the game (textures, maps, etc.)

	let mut world = World {
		player: Default::default(),
		enemies: Default::default(),
		npcs: Default::default(),
		attacks: Default::default(),

		config: Config::read("./config.ron"),
		current_map: String::from("default:test"),
		hitstop: 0.
	};

	world.player.insert(Player::new());
	world.populate();

	loop {
		update_delta_time();
		draw(&mut world).await;
		
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();

			darken_screen();
			next_frame().await;
			continue;
		}

		// NPC Dialogue (WIP)
		for (obj, messages, messages_cooldown) in query!(world.npcs, (&obj, &messages, &mut messages_cooldown)) {
			if *messages_cooldown > 0. { 
				*messages_cooldown -= get_delta_time();
				continue 
			}

			for (atk_obj, atk_type, owner) in query!(world.attacks, (&obj, &attack_type, &owner)) {
				if !atk_obj.is_touching(obj) 
				|| *atk_type == AttackType::Projectile
				|| *atk_type == AttackType::Hitscan
				|| *owner != Owner::Player {
					continue;
				}

				for message in messages {
					if message.should_read() {
						message.read();
						*messages_cooldown = 10.;
						break
					}
				}
			}
		}

		// Attacking
		handle_combat(&mut world);

		for (inventory, obj) in query!(world.player, (&mut inventory, &obj)) {
			// Switching weapons
			if world.config.keymap.change_sword.is_pressed() {
				inventory.current_sword = swap_weapons(&inventory.current_sword, &inventory.swords);
			}
			if world.config.keymap.change_gun.is_pressed() {
				inventory.current_gun = swap_weapons(&inventory.current_gun, &inventory.guns);
			}

			// Cooldown
			for sword in inventory.swords.iter_mut() {
				if sword.cooldown >= 0. {
					sword.cooldown -= get_delta_time()
				}
			}
			for gun in inventory.guns.iter_mut() {
				if gun.cooldown >= 0. {
					gun.cooldown -= get_delta_time()
				}
			}

			// Creating attacks
			if world.config.keymap.sword.is_down() && inventory.swords[inventory.current_sword].cooldown <= 0. {
				world.attacks.insert(inventory.attack_sword(obj.pos)); 
			}
			if world.config.keymap.gun.is_down() && inventory.guns[inventory.current_gun].cooldown <= 0. {
				world.attacks.insert(inventory.attack_gun(obj.pos)); 
			}
		}

		// Updating health (this is primarily for i-frames)
		for hp in query!([world.player, world.enemies], (&mut health)) {
			hp.update();
		}

		// Movement and behavior
		handle_behavior(&mut world);
		
		// handling dead entities/players and old attacks 
		remove_dead_enemies(&mut world);
		remove_old_attacks(&mut world);
		try_player_death(&mut world);

		// Quitting the game
		if world.config.keymap.quit.is_down() {
			unsafe{ clean_resources(); }
			return State::Menu
		}

		next_frame().await
	}
}

/// Handling dead enemies.
/// TODO: Death animation
fn remove_dead_enemies(world: &mut World) {
	let mut to_remove: usize = 0;
	while {
		let mut enemy_to_remove = false;

		for (index, enemy) in world.enemies.iter() {
			if enemy.health.should_kill() {
				to_remove = index;
				enemy_to_remove = true;
				break;
			}
		}

		enemy_to_remove
	} {
		world.enemies.remove(to_remove);
	}
}

/// Handling old attacks
fn remove_old_attacks(world: &mut World) {
	let mut to_remove: usize = 0;
	while {
		let mut atk_to_remove = false;

		for (index, atk) in world.attacks.iter() {
			if match atk.attack_type {
				AttackType::Physical | AttackType::Burst => atk.sprite.anim_completed(),
				_ => *atk.lifetime <= 0.
			} {
				to_remove = index;
				atk_to_remove = true;
				break;
			}
		}

		atk_to_remove
	} { world.attacks.remove(to_remove); }
}

/// Handling the player's death (WIP)
fn try_player_death(world: &mut World) {
	let mut player_is_dead = false;

	for hp in query!(world.player, (&health)) {
		if hp.should_kill() {
			player_is_dead = true;
			break;
		}
	}

	if player_is_dead {
		while !world.player.ids.is_empty() {
			world.player.remove(0);
		}

		world.player.insert(Player::new());

		world.populate();
	}
}
