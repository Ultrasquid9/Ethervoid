use draw::{draw, render::darken_screen};
use macroquad::prelude::*;
use stecs::prelude::*;

use crate::{
	State,
	utils::{config::Config, get_delta_time, resources::create_resources, update_delta_time},
};

use combat::{AttackType, Owner, handle_combat};

use ecs::{World, behavior::handle_behavior};

use player::{Player, swap_weapons};

pub mod combat;
pub mod doors;
pub mod draw;
pub mod ecs;
pub mod enemy;
pub mod npc;
pub mod player;

pub struct Gameplay {
	pub world: World,
	pub hitstop: f64,
	pub current_map: String,
	pub config: Config, // TODO: Make Global
}

impl Gameplay {
	fn new() -> Gameplay {
		Gameplay {
			world: World {
				player: Default::default(),
				enemies: Default::default(),
				npcs: Default::default(),
				attacks: Default::default(),
			},
			hitstop: 0.,
			current_map: String::from("default:test"),
			config: Config::read("./config.ron"),
		}
	}

	fn npc_dialogue(&mut self) {
		for (obj, messages, messages_cooldown) in
			query!(self.world.npcs, (&obj, &messages, &mut messages_cooldown))
		{
			if *messages_cooldown > 0. {
				*messages_cooldown -= get_delta_time();
				continue;
			}

			for (atk_obj, atk_type, owner) in
				query!(self.world.attacks, (&obj, &attack_type, &owner))
			{
				if !atk_obj.is_touching(obj)
					|| *atk_type == AttackType::Projectile
					|| *atk_type == AttackType::Hitscan
					|| *owner != Owner::Player
				{
					continue;
				}

				for message in messages {
					if message.should_read() {
						message.read();
						*messages_cooldown = 10.;
						break;
					}
				}
			}
		}
	}

	fn change_weapon(&mut self) {
		for (inventory, obj) in query!(self.world.player, (&mut inventory, &obj)) {
			// Switching weapons
			if self.config.keymap.change_sword.is_pressed() {
				inventory.current_sword = swap_weapons(&inventory.current_sword, &inventory.swords);
			}
			if self.config.keymap.change_gun.is_pressed() {
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
			if self.config.keymap.sword.is_down()
				&& inventory.swords[inventory.current_sword].cooldown <= 0.
			{
				self.world.attacks.insert(inventory.attack_sword(obj.pos));
			}
			if self.config.keymap.gun.is_down()
				&& inventory.guns[inventory.current_gun].cooldown <= 0.
			{
				self.world.attacks.insert(inventory.attack_gun(obj.pos));
			}
		}
	}

	fn update_health(&mut self) {
		for hp in query!([self.world.player, self.world.enemies], (&mut health)) {
			hp.update();
		}
	}

	/// Handling the player's death (WIP)
	fn try_player_death(&mut self) {
		let mut player_is_dead = false;

		for hp in query!(self.world.player, (&health)) {
			if hp.should_kill() {
				player_is_dead = true;
				break;
			}
		}

		if player_is_dead {
			while !self.world.player.ids.is_empty() {
				self.world.player.remove(0);
			}

			self.world.player.insert(Player::new());

			self.world.populate(&self.current_map);
		}
	}

	/// Handling old attacks
	fn remove_old_attacks(&mut self) {
		let mut to_remove: usize = 0;
		while {
			let mut atk_to_remove = false;

			for (index, atk) in self.world.attacks.iter() {
				if match atk.attack_type {
					AttackType::Physical | AttackType::Burst => atk.sprite.anim_completed(),
					_ => *atk.lifetime <= 0.,
				} {
					to_remove = index;
					atk_to_remove = true;
					break;
				}
			}

			atk_to_remove
		} {
			self.world.attacks.remove(to_remove);
		}
	}

	/// Handling dead enemies.
	/// TODO: Death animation
	fn remove_dead_enemies(&mut self) {
		let mut to_remove: usize = 0;
		while {
			let mut enemy_to_remove = false;

			for (index, enemy) in self.world.enemies.iter() {
				if enemy.health.should_kill() {
					to_remove = index;
					enemy_to_remove = true;
					break;
				}
			}

			enemy_to_remove
		} {
			self.world.enemies.remove(to_remove);
		}
	}
}

pub async fn gameplay() -> State {
	// Locates and creates all the resources in the game (textures, maps, etc.)
	create_resources();

	let mut gameplay = Gameplay::new();

	gameplay.world.player.insert(Player::new());
	gameplay.world.populate(&gameplay.current_map);

	loop {
		update_delta_time();
		draw(&mut gameplay).await;

		// Handling hitstop
		if gameplay.hitstop > 0. {
			gameplay.hitstop -= get_delta_time();

			darken_screen();
			next_frame().await;
			continue;
		}

		handle_combat(&mut gameplay);
		handle_behavior(&mut gameplay);

		gameplay.npc_dialogue();
		gameplay.change_weapon();
		gameplay.update_health();
		gameplay.remove_dead_enemies();
		gameplay.remove_old_attacks();
		gameplay.try_player_death();

		// Quitting the game
		if gameplay.config.keymap.quit.is_down() {
			return State::Menu;
		}

		next_frame().await
	}
}
