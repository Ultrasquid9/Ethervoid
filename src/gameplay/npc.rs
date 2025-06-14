use macroquad::math::DVec2;
use messages::Message;
use rustc_hash::FxHashMap;
use stecs::prelude::*;

use super::ecs::{
	behavior::{Behavior, wander::WanderBehavior},
	obj::Obj,
	sprite::{Frames, Rotation, Sprite},
};

use crate::{
	cores::npctype::{NpcMovement, NpcType},
	utils::ImmutVec,
};

pub mod messages;

#[derive(SplitFields)]
pub struct Npc {
	obj: Obj,
	behavior: Behavior,
	sprite: Sprite,

	messages: ImmutVec<Message>,
	messages_cooldown: f64,
}

impl Npc {
	pub fn from_type(npctype: &NpcType, pos: &DVec2) -> Self {
		let obj = Obj::new(*pos, *pos, 15.);

		Self {
			obj,
			behavior: match npctype.movement {
				NpcMovement::Wander(range) => Behavior::Wander(WanderBehavior {
					pos: *pos,
					range,
					cooldown: 0.,
				}),
				NpcMovement::Still => Behavior::None,
			},
			sprite: Sprite::new(
				obj,
				"default:entity/player/player_spritesheet_wip",
				Rotation::EightWay,
				Frames::new_entity(),
				FxHashMap::default(),
			),

			messages: npctype.messages.clone(),
			messages_cooldown: 10.,
		}
	}
}
