use ahash::HashMap;
use macroquad::math::Vec2;
use messages::Message;
use stecs::prelude::*;

use super::ecs::{
	sprite::{
		Frames, 
		Rotation, 
		Sprite
	},
	behavior::{
		Behavior, 
		WanderBehavior
	}, 
	obj::Obj
};

use crate::cores::npctype::{
	NpcMovement, 
	NpcType
};

pub mod messages;

#[derive(SplitFields)]
pub struct Npc {
	obj: Obj,
	behavior: Behavior,
	sprite: Sprite,
	
	messages: Box<[Message]>,
	messages_cooldown: f32,
}

impl Npc {
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
				Frames::new_entity(),
				HashMap::default()
			),

			messages: npctype.messages.clone(),
			messages_cooldown: 0.,
		}
	}
}
