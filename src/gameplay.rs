use combat::{try_parry, Attack};
use draw::draw;
use ecs::{behavior::handle_behavior, World};
use enemy::Enemy;
use macroquad::window::next_frame;
use npc::Npc;
use player::Player;
use stecs::prelude::*;

use crate::{utils::{get_delta_time, resources::{create_resources, maps::access_map}}, State};

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

		current_map: String::from("default:test"),
		hitstop: 0.
	};

	let mut attacks: Vec<Attack> = Vec::new();

	for (enemy, pos) in access_map(&world.current_map).enemies.iter() {
		let _ = world.enemies.insert(Enemy::from_type(enemy, pos));
	}

	for (npc, pos) in access_map(&world.current_map).npcs.iter() {
		let _ = world.npcs.insert(Npc::from_type(npc, pos));
	}

	world.player.insert(Player::new());

	loop {
		draw(&mut world, &mut attacks).await;
		
		// Handling hitstop
		if world.hitstop > 0. {
			world.hitstop -= get_delta_time();

			next_frame().await;
			continue;
		}

		// Attacking
		for (inventory, config, obj) in query!(world.player, (&mut inventory, &config, &obj)) {
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
			if config.keymap.sword.is_down() && inventory.swords[inventory.current_sword].cooldown <= 0. {
				attacks.push(inventory.attack_sword(obj.pos)); 
			}
			if config.keymap.gun.is_down() && inventory.guns[inventory.current_gun].cooldown <= 0. {
				attacks.push(inventory.attack_gun(obj.pos)); 
			}
		}

		// Attacks 
		handle_behavior(&mut world, &mut attacks);
		try_parry(&mut attacks, &mut world);
		attacks.retain(|atk| !atk.should_rm());

		for atk in attacks.iter_mut() {
			atk.update(&mut world);
		}

		next_frame().await
	}
}
