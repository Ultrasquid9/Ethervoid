use stecs::prelude::*;

use macroquad::math::Vec2;

use serde::{
	Deserialize, 
	Serialize
};

use crate::{cores::npctype::{Message, NpcType}, utils::get_delta_time};

use super::ecs::{behavior::Behavior, obj::Obj};

#[derive(SplitFields)]
pub struct Npc<'a> {
	obj: Obj,
	behavior: Behavior<'a>,
	
	messages: Vec<Message>,
	messages_cooldown: f32,

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
pub enum NpcMovement {
	Wander(f32),
	Still
}

impl Npc<'_> {
	pub fn new(npctype: NpcType, pos: Vec2) -> Self {
		return Self {
			obj: Obj::new(pos, pos, 15.),
			behavior: Behavior::Script(npctype.movement.build()),

			messages: npctype.messages,
			messages_cooldown: 0.,

			movement_cooldown: 0.,
			movement_target: pos
		}
	}

	pub fn update(&mut self) {
		if self.messages_cooldown <= 0. {
			self.read_message();
		} else {
			self.messages_cooldown -= get_delta_time()
		}
	}

	pub fn read_message(&mut self) {
		let mut should_read = false;

		if !should_read { return }

		for i in &self.messages {
			if i.should_read() {
				i.read();
				self.messages_cooldown = 10.;
				break
			}
		}
	}
}

impl Dialogue {
	/// Reading dialogue
	/// Note: HIGHLY WIP
	pub fn read(&self) { println!("{}", self.text) } 
}
