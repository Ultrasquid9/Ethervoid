use macroquad::math::Vec2;

use super::{combat::Attack, cores::{attackscript::AttackScript, enemytype::EnemyType}, entity::Entity, player::Player};

/// The movement AI used by an enemy
#[derive(Clone)]
pub enum Movement {
	MoveTowardsPlayer
}

impl Movement {
	/// Provides a Movement enum based on the provided String
	pub fn from_str(input: &str) -> Movement {
		match input {
			"MoveTowardsPlayer" => Movement::MoveTowardsPlayer,

			_ => Movement::MoveTowardsPlayer
		}
	}
}

/// An enemy
pub struct Enemy {
	pub stats: Entity,
	movement: Movement,

	id: usize,
	attacks: Vec<AttackScript>,
	current_attack: Option<AttackScript>,
	attack_index: usize,
	attack_cooldown: usize
}

impl Enemy {
	/// Creates a new Enemy using a Vec2 for the pos and an EnemyType for the stats
	pub fn new(pos: Vec2, enemytype: EnemyType, id: usize) -> Self {
		return Self {
			stats: Entity::new(pos, enemytype.size, enemytype.max_health as isize),
			movement: enemytype.movement,

			id,
			attacks: enemytype.attacks,
			current_attack: None,
			attack_index: 0,
			attack_cooldown: 32
		}
	}

	/// Updates the enemy based upon their AI and the Player's stats
	pub fn update(&mut self, player: &mut Player, map: &Vec<Vec2>, attacks: &mut Vec<Attack>) {
		if self.stats.i_frames != 0 {
			self.stats.i_frames -= 1
		}

		match &self.current_attack {
			Some(_) => {
				if self.current_attack.clone().unwrap().read_script(self, player, map, attacks) {
					self.current_attack = None;
					self.attack_cooldown = 64;
				}
			}
			None => {
				if self.attack_cooldown == 0 {
					self.current_attack = Some(self.attacks[self.attack_index].clone());
					self.current_attack.as_mut().unwrap().set_target(player.stats.get_pos());
					
					if self.attack_index == self.attacks.len() - 1 {
						self.attack_index = 0;
					} else {
						self.attack_index += 1;
					}
					self.attack_cooldown = 64;
				} else {
					self.movement(player, map);
					self.attack_cooldown -= 1;
				}
			}
		}
	}

	/// Moves the enemy based upon their Movement
	fn movement(&mut self, player: &Player, map: &Vec<Vec2>){
		match self.movement {
			// Simple movement AI that tracks the player and moves towards them
			Movement::MoveTowardsPlayer => {
				self.stats.try_move(self.stats.get_pos().move_towards(player.stats.get_pos(), 1.0), map);
			}
		}
	}

	/// Gets the ID of the enemy
	pub fn get_id(&self) -> usize {
		return self.id
	}
}
