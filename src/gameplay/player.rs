use stecs::prelude::*;

use super::ecs::{behavior::Behavior, health::Health, obj::Obj};

#[derive(SplitFields)]
pub struct Player {
    health: Health,
    obj: Obj,
    behavior: Behavior
}
