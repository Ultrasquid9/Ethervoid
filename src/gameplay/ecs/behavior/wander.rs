use macroquad::{math::DVec2, rand::gen_range};

use crate::{gameplay::ecs::obj::Obj, utils::smart_time};

#[derive(PartialEq, Clone)]
pub struct WanderBehavior {
	pub pos: DVec2,
	pub range: f64,

	pub cooldown: f64,
}

pub fn wander_behavior(behavior: &mut WanderBehavior, obj: &mut Obj, current_map: &str) {
	if behavior.cooldown > 0. {
		behavior.cooldown -= smart_time();
		return;
	} else if obj.pos.distance(obj.target) < 5. {
		obj.update(DVec2::new(
			gen_range(
				behavior.pos.x - behavior.range,
				behavior.pos.x + behavior.range,
			),
			gen_range(
				behavior.pos.y - behavior.range,
				behavior.pos.y + behavior.range,
			),
		));
		behavior.cooldown = 120.;
		return;
	}
	let new_pos = obj.pos.move_towards(obj.target, 2. * smart_time());
	obj.try_move(&new_pos, current_map);

	if obj.pos != new_pos {
		obj.target = obj.pos;
	}
}
