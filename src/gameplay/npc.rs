use macroquad::{math::Vec2, prelude::rand};
use serde::{Deserialize, Serialize};

use super::{cores::{map::Map, npctype::{Message, NPCType}}, draw::{access_texture, texturedobj::{EntityTexture, TexturedObj}}, entity::MovableObj};

pub struct NPC {
	pos: Vec2,
	center_pos: Vec2,
	messages: Vec<Message>,
	movement: NPCMovement,
	
	pub texture: EntityTexture,

	movement_cooldown: f32,
	movement_target: Vec2
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
	name: String,
	portrait: String,
	text: String
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
			center_pos: pos,
			messages: npctype.messages,
			movement: npctype.movement,
			texture: EntityTexture::new(access_texture(&npctype.sprite)),

			movement_cooldown: 0.,
			movement_target: pos
		}
	}

	pub fn update(&mut self, map: &Map) {
		self.update_texture();
		self.movement(map);

		for i in &self.messages {
			if i.should_read() {
				i.read();
				break
			}
		}
	}

	fn movement(&mut self, map: &Map) {
		match self.movement {
			NPCMovement::Wander(range) => {
				if self.pos.distance(self.movement_target) < 5. {
					self.movement_target = Vec2::new(
						rand::gen_range(self.center_pos.x - range, self.center_pos.x + range),
						rand::gen_range(self.center_pos.y - range, self.center_pos.y + range)
					);
					self.movement_cooldown = 100.
				} else if self.movement_cooldown <= 0. {
					let new_pos = self.pos.move_towards(self.movement_target, 2.);

					self.try_move(new_pos, map);

					if self.pos != new_pos {
						self.movement_target = self.pos
					}
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

impl TexturedObj for NPC {
	fn update_texture(&mut self) {
		self.texture.update(
			self.pos.clone(), 
			super::player::Axis::Positive, 
			super::player::Axis::Positive, 
			true
		);
	}
}

impl Dialogue {
	/// Reading dialogue
	/// Note: HIGHLY WIP
	pub fn read(&self) { println!("{}", self.text) } 
}
