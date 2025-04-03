use draw::{draw, render::darken_screen};
use macroquad::prelude::*;
use npc::messages::Message;
use stecs::prelude::*;

use crate::{
	State,
	menu::{dialogue, pause},
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
	pub current_map: String,
	pub current_message: Option<Message>,
	pub hitstop: f64,
	pub paused: bool,
	pub config: Config, // TODO: Make Global
}

impl Gameplay {
	fn new() -> Gameplay {
		Gameplay {
			world: World::new(),
			current_map: String::from("default:test"),
			current_message: None,
			hitstop: 0.,
			paused: false,
			config: Config::read("./config.ron"),
		}
	}

	fn should_darken(&self) -> bool {
		self.hitstop > 0. || self.paused || self.current_message.is_some()
	}

	fn pause(&mut self) {
		if self.current_message.is_some() {
			return;
		}

		if self.config.keymap.pause.is_pressed() {
			self.paused = !self.paused;
		}
	}

	fn get_npc_dialogue(&mut self) {
		if self.current_message.is_some() {
			return;
		}

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
						self.current_message = Some(message.clone());
						*messages_cooldown = 10.;
						break;
					}
				}
			}
		}
	}

	async fn read_npc_dialogue(&mut self) {
		let Some(message) = &mut self.current_message else {
			return;
		};

		dialogue::menu(message).await;

		if message.should_stop() {
			self.current_message = None;
		}
	}

	fn hitstop(&mut self) {
		if self.hitstop > 0. {
			self.hitstop -= get_delta_time();
		}
	}

	fn change_weapon(&mut self) {
		for (inventory, obj) in query!(self.world.player, (&mut inventory, &obj)) {
			// Switching weapons
			if self.config.keymap.change_sword.is_pressed() {
				inventory.current_sword = swap_weapons(inventory.current_sword, &inventory.swords);
			}
			if self.config.keymap.change_gun.is_pressed() {
				inventory.current_gun = swap_weapons(inventory.current_gun, &inventory.guns);
			}

			// Cooldown
			for sword in &mut inventory.swords {
				if sword.cooldown >= 0. {
					sword.cooldown -= get_delta_time();
				}
			}
			for gun in &mut inventory.guns {
				if gun.cooldown >= 0. {
					gun.cooldown -= get_delta_time();
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

		// Anything that pauses normal gameplay goes here
		gameplay.pause();
		gameplay.get_npc_dialogue();

		if gameplay.should_darken() {
			gameplay.read_npc_dialogue().await;
			gameplay.hitstop();

			if gameplay.paused {
				let state = pause::menu().await;

				if let Some(state) = state {
					match state {
						State::Gameplay => gameplay.paused = false,
						state => return state,
					}
				}
			}

			darken_screen();
			next_frame().await;
			continue;
		}

		// Normal gameplay continues
		handle_combat(&mut gameplay);
		handle_behavior(&mut gameplay);

		gameplay.change_weapon();
		gameplay.update_health();
		gameplay.remove_dead_enemies();
		gameplay.remove_old_attacks();
		gameplay.try_player_death();

		next_frame().await;
	}
}
