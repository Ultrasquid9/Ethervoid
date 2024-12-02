use stecs::prelude::*;

use macroquad::math::Vec2;

use serde::{
	Deserialize, 
	Serialize
};

use crate::cores::npctype::{Message, NpcMovement, NpcType};

use super::ecs::{behavior::{Behavior, WanderBehavior}, obj::Obj, sprite::{Frames, Rotation, Sprite}};

#[derive(SplitFields)]
pub struct Npc<'a> {
	obj: Obj,
	behavior: Behavior<'a>,
	sprite: Sprite,
	
	messages: Vec<Message>,
	messages_cooldown: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
	name: String,
	portrait: String,
	text: String
}

impl Npc<'_> {
	pub fn from_type(npctype: &NpcType, pos: &Vec2) -> Self {
		let obj = Obj::new(*pos, *pos, 15.);
		
		Self {
			obj,
			behavior: match npctype.movement {
				NpcMovement::Wander(range) => Behavior::Wander(WanderBehavior {
					pos: *pos,
					range,
					cooldown: 0.
				}),
				NpcMovement::Still => Behavior::None
			},
			sprite: Sprite::new(
				obj, 
				32,
				"default:entity/player/player_spritesheet_wip",
				Rotation::EightWay,
				Frames::new_entity()
			),

			messages: npctype.messages.clone(),
			messages_cooldown: 0.,
		}
	}
}

impl Dialogue {
	/// Reading dialogue
	/// Note: HIGHLY WIP
	pub fn read(&self) { println!("{}", self.text) } 
}
