use combat::{handle_combat, AttackType, Owner};
use draw::draw;
use ecs::{behavior::handle_behavior, World};
use macroquad::window::next_frame;
use player::{swap_weapons, Player};
use stecs::prelude::*;

use crate::{utils::{config::Config, get_delta_time, resources::{clean_resources, create_resources}}, State};

pub mod combat;
pub mod doors;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub async fn gameplay() -> State {
	unsafe { create_resources(); } // TODO: Clean resources (irrelevant until main menu is reimplemented)

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
		draw(&mut world).await;
		
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();

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
		
		// Removing dead enemies and old attacks
		remove_dead_enemies(&mut world);
		remove_old_attacks(&mut world);

		// Quitting the game
		if world.config.keymap.quit.is_down() {
			unsafe{ clean_resources(); }
			return State::Quit
		}

		next_frame().await
	}
}

fn remove_dead_enemies(world: &mut World) {
	let mut to_remove: usize = 0;
	while (|| {
		for (index, enemy) in world.enemies.iter() {
			if enemy.health.should_kill() {
				to_remove = index;
				return true
			}
		}
		false
	})() {
		world.enemies.remove(to_remove);
	}
}

fn remove_old_attacks(world: &mut World) {
	let mut to_remove: usize = 0;
	while (|| {
		for (index, atk) in world.attacks.iter() {
			if match atk.attack_type {
				AttackType::Physical | AttackType::Burst => atk.sprite.anim_completed(),
				_ => *atk.lifetime <= 0.
			} {
				to_remove = index;
				return true
			}
		}
		false
	})() {
		world.attacks.remove(to_remove);
	}
}
