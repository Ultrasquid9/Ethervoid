use macroquad::{
	math::Vec2, 
	prelude::rand
};

use serde::{
	Deserialize, 
	Serialize
};

use crate::utils::{
	resources::access_texture,
	get_delta_time
};

use super::{
	cores::{ 
		npctype::{
			Message,
			NPCType
		},
		map::Map
	}, 
	combat::{
		AttackType, 
		Owner
	},
	draw::texturedobj::{
		EntityTexture, 
		TexturedObj
	}, 
	entity::{
		get_axis, 
		MovableObj
	}, 
	ecs::Attacks, 
	player::Axis
};

pub struct NPC {
	pos: Vec2,
	center_pos: Vec2,
	
	pub texture: EntityTexture,

	messages: Vec<Message>,
	messages_cooldown: f32,

	movement: NPCMovement,
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

			texture: EntityTexture::new(access_texture(&npctype.sprite)),

			messages: npctype.messages,
			messages_cooldown: 0.,

			movement: npctype.movement,
			movement_cooldown: 0.,
			movement_target: pos
		}
	}

	pub fn update(&mut self, map: &Map, attacks: &Attacks) {
		self.update_texture();
		self.movement(map);

		if self.messages_cooldown <= 0. {
			self.read_message(attacks);
		} else {
			self.messages_cooldown -= get_delta_time()
		}
	}

	pub fn read_message(&mut self, attacks: &Attacks) {
		let mut should_read = false;
		for i in &attacks.io {
			if i.attack_type == AttackType::Physical
			&& i.owner == Owner::Player
			&& i.is_touching(self) {
				should_read = true;
				break
			}
		}

		if !should_read { return }

		for i in &self.messages {
			if i.should_read() {
				i.read();
				self.messages_cooldown = 10.;
				break
			}
		}
	}

	fn movement(&mut self, map: &Map) {
		match self.movement {
			NPCMovement::Wander(range) => {
				if self.movement_cooldown > 0. {
					self.movement_cooldown -= get_delta_time()

				} else if self.pos.distance(self.movement_target) < 5. {
					self.movement_target = Vec2::new(
						rand::gen_range(self.center_pos.x - range, self.center_pos.x + range),
						rand::gen_range(self.center_pos.y - range, self.center_pos.y + range)
					);
					self.movement_cooldown = 120.

				} else {
					let new_pos = self.pos.move_towards(self.movement_target, 2.);

					self.try_move(new_pos, map);

					if self.pos != new_pos {
						self.movement_target = self.pos
					}
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
		let moving = if self.movement_cooldown >= 0. {
			false
		} else {
			true
		};

		let new_axis = if moving {
			get_axis(self.pos, self.movement_target)
		} else {
			(Axis::None, Axis::Negative)
		};

		self.texture.update(
			self.pos.clone(), 
			new_axis.0, 
			new_axis.1, 
			moving
		);
	}
}

impl Dialogue {
	/// Reading dialogue
	/// Note: HIGHLY WIP
	pub fn read(&self) { println!("{}", self.text) } 
}
