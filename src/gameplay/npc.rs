use macroquad::{math::Vec2, rand::rand};
use serde::{Deserialize, Serialize};

use super::{cores::{map::Map, npctype::{Message, NPCType}}, draw::{access_texture, texturedobj::EntityTexture}, entity::MovableObj};

pub struct NPC {
	pos: Vec2,
	messages: Vec<Message>,
	movement: NPCMovement,
	
	pub texture: EntityTexture,

	movement_cooldown: f32,
	movement_target: Vec2
}

#[derive(Clone, Serialize, Deserialize)]
pub enum NPCMovement {
	Wander(f32),
	Still
}

impl NPC {
	pub fn new(npctype: NPCType, pos: Vec2) -> Self {
		Self {
			pos,
			messages: npctype.messages,
			movement: npctype.movement,
			texture: EntityTexture::new(access_texture(&npctype.sprite)),

			movement_cooldown: 0.,
			movement_target: pos
		}
	}

	pub fn update(&mut self, map: &Map) {
		self.movement(map);
	}

	fn movement(&mut self, map: &Map) {
		match self.movement {
			NPCMovement::Wander(range) => {
				if self.pos.distance(self.movement_target) < 5. {
					self.movement_target = Vec2::new(
						(rand() as f32).clamp(self.pos.x - range, self.pos.x + range), 
						(rand() as f32).clamp(self.pos.x - range, self.pos.x + range)
					);
					self.movement_cooldown = 60.
				} else if self.movement_cooldown <= 0. {
					self.try_move(self.pos.move_towards(self.movement_target, 2.), map);
				} else {
					self.movement_cooldown -= 1.
				}
			},
			NPCMovement::Still => ()
		}
	}
}

// Allows the NPC to move
impl MovableObj for NPC {
	fn get_size(&self) -> &f32 { &15. } // NPC size is hardcoded for now

	fn get_pos(&self) -> Vec2 {
		return self.pos
	}

	fn edit_pos(&mut self) -> &mut Vec2 {
		&mut self.pos
	}
}
