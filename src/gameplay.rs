use combat::{handle_combat, try_parry, AttackType};
use draw::draw;
use ecs::{behavior::handle_behavior, World};
use enemy::Enemy;
use macroquad::window::next_frame;
use npc::Npc;
use player::Player;
use stecs::prelude::*;

use crate::{utils::{config::Config, get_delta_time, resources::{clean_resources, create_resources, maps::access_map}}, State};

pub mod combat;
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

	for (enemy, pos) in access_map(&world.current_map).enemies.iter() {
		let _ = world.enemies.insert(Enemy::from_type(enemy, pos));
	}

	for (npc, pos) in access_map(&world.current_map).npcs.iter() {
		let _ = world.npcs.insert(Npc::from_type(npc, pos));
	}

	world.player.insert(Player::new());

	loop {
		draw(&mut world).await;
		
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();

			next_frame().await;
			continue;
		}

		// Attacking
		for (inventory, obj) in query!(world.player, (&mut inventory, &obj)) {
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

		// Attacks 
		handle_behavior(&mut world);
		handle_combat(&mut world);
		
		try_parry(&mut world);

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
		return false
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
		return false
	})() {
		world.attacks.remove(to_remove);
	}
}
