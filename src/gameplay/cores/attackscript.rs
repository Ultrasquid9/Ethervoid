use std::{collections::HashMap, fs};

use macroquad::math::{vec2, Vec2};
use rhai::{Dynamic, Engine, Scope};
use serde::Deserialize;

use crate::gameplay::{combat::{Attack, Owner}, draw::texturedobj::AttackTextureType, entity::{Entity, MovableObj}, player::Player};

use super::{gen_name, get_files};

#[derive(Clone, Deserialize)]
pub struct AttackScriptBuilder (String);

impl AttackScriptBuilder {
	/// Creates an attack with the script at the provided directory
	pub fn from(dir: String) -> Self {
		Self(fs::read_to_string(dir).unwrap())
	}

	/// Creates an attack with the script at the provided directory
	pub fn build<'a>(self) -> AttackScript<'a> {
		AttackScript {
			current_target: Vec2::new(0., 0.),
			script: self.0,
			scope: Scope::new(),
			engine: Engine::new()
		}
	}
}

/// An Attack that can be configured via a script
/// The lifetime annotation allows the compiler to know that the AttackScript lives as long as the Enemy does
pub struct AttackScript<'a> {
	pub current_target: Vec2,
	script: String,
	scope: Scope<'a>,
	engine: Engine
}

impl AttackScript<'_> {
	/// Reads the attack script. Returns true if the enemy has reached the target, or if the enemy could not move
	pub fn read_script<'a>(&mut self, entity: &'a mut Entity, player: &Player, map: &Vec<Vec2>, attacks: &mut Vec<Attack>) -> bool {
		// Values available in the scope
		self.scope
			.push("attacks", Vec::<Dynamic>::new())
			.push_constant("player_pos", player.stats.get_pos().clone())
			.push_constant("enemy_pos", entity.get_pos().clone())
			.push_constant("target_pos", self.current_target.clone());

		// Values needed for the script, but not exposed to it 
		let entity_pos = entity.get_pos();

		// The Vec2 built-in methods don't work, so I have to make shitty copies
		fn move_towards(pos1: Vec2, pos2: Vec2, distance: f32) -> Vec2 {
			return pos1.move_towards(pos2, distance)
		}
		fn distance_between(pos1: Vec2, pos2: Vec2) -> f32 {
			return pos1.distance(pos2)
		}

		// Registerring functions for the script
		self.engine
			// Registerring the Vec2 and functions related to it
			.register_type_with_name::<Vec2>("position")
			.register_fn("move_towards", move_towards)
			.register_fn("distance_between", distance_between)

			// Functions for creating attacks
			.register_fn("new_physical", move |damage: i64, size, target: Vec2,| Attack::new_physical(
				entity_pos, 
				target,
				damage as isize, 
				size, 
				Owner::Enemy,
				AttackTextureType::Dash
			))
			.register_fn("new_burst", move |damage: i64, size| Attack::new_burst(
				entity_pos, 
				damage as isize, 
				size, 
				Owner::Enemy
			))
			.register_fn("new_projectile", move |damage: i64, target: Vec2| Attack::new_projectile(
				entity_pos, 
				target,
				damage as isize, 
				Owner::Enemy,
				AttackTextureType::ProjectileEnemy
			))
			.register_fn("new_hitscan", move |damage: i64, target: Vec2| Attack::new_hitscan(
				entity_pos, 
				target,
				damage as isize, 
				Owner::Enemy
			))

			// Hacky method to end the script
			.register_fn("end", move || Vec2::new(999999., 999999.));

		// Executing the script
		let new_pos = match self.engine.eval_with_scope::<Vec2>(&mut self.scope, &self.script) {
			Ok(new_pos) => new_pos,
			Err(e) => panic!("Bad script: {}", e)
		};

		// Getting attacks out of the scope
		let new_attacks = self.scope
			.get_value_mut::<Vec<Dynamic>>("attacks")
			.expect("Attacks not found");
		for i in new_attacks {
			attacks.push(i.clone_cast())
		}

		// A horrible hacky way of checking if the 'end' keyword was called
		if new_pos == vec2(999999., 999999.) {
			return true;
		} else {
			entity.update_axis(&new_pos);
			entity.try_move(new_pos, map);
		}

		// Returns true if the enemy could not move or if the enemy has reached the target
		// Otherwise, returns false
		if entity.get_pos() == self.current_target
		|| entity.get_pos() != new_pos {
			return true
		} else {
			return false
		}
	}
}

/// Provides a HashMap containing all Attacks
pub fn get_attacks() -> HashMap<String, AttackScriptBuilder> {
	let mut attacks: HashMap<String, AttackScriptBuilder> = HashMap::new();

	for i in get_files(String::from("attacks")) {
		attacks.insert(
			gen_name(&i),
			AttackScriptBuilder::from(i)
		);
	}

	return attacks;
}
