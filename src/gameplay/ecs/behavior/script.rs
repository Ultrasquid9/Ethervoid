use macroquad::math::{vec2, Vec2};
use rhai::Dynamic;

use crate::{cores::script::Script, gameplay::{combat::{Attack, Owner}, ecs::obj::Obj}, utils::get_delta_time};

/// Reads a script
pub fn script_movement(
	script: &mut Script<'_>, 
	obj: &mut Obj, 
	obj_player: &Obj,
	_attacks: &mut Vec<Attack>
) {
	// Values available in the scope
	script.scope
		.push("attacks", Vec::<Dynamic>::new())
		.push_constant("player_pos", obj_player.pos)
		.push_constant("enemy_pos", obj.pos)
		.push_constant("target_pos", script.current_target);

	// Values needed for the script, but not exposed to it
	let mut obj = obj.clone();

	// The Vec2 built-in methods don't work, so I have to make shitty copies
	fn move_towards(pos1: Vec2, pos2: Vec2, distance: f32) -> Vec2 {
		pos1.move_towards(pos2, distance)
	}
	fn distance_between(pos1: Vec2, pos2: Vec2) -> f32 {
		pos1.distance(pos2)
	}

	// Registerring functions for the script
	script.engine
		// Registerring the Vec2 and functions related to it
		.register_type_with_name::<Vec2>("position")
		.register_fn("move_towards", move_towards)
		.register_fn("distance_between", distance_between)

		// Functions for creating attacks
		.register_fn("new_physical", move |damage: f32, size, target: Vec2,| return Attack::new_physical(
			Obj::new(obj.pos, target, size),
			damage, 
			Owner::Enemy,
			"default:attacks/dash"
		))
		.register_fn("new_burst", move |damage: f32, size| return Attack::new_burst(
			Obj::new(obj.pos, obj.pos, size), 
			damage, 
			Owner::Enemy,
			"default:attacks/burst"
		))
		.register_fn("new_projectile", move |damage: f32, target: Vec2| return Attack::new_projectile(
			Obj::new(obj.pos, target, 10.),
			damage, 
			Owner::Enemy,
			"default:attacks/projectile-enemy"
		))
		.register_fn("new_hitscan", move |damage: f32, target: Vec2| return Attack::new_hitscan(
			Obj::new(obj.pos, target, 6.),
			damage, 
			Owner::Enemy
		))

		// Hacky method to end the script
		.register_fn("end", move || Vec2::new(999999., 999999.));

	// Executing the script
	let new_pos = match script.engine.eval_with_scope::<Vec2>(&mut script.scope, &script.script) {
		Ok(new_pos) => new_pos,
		Err(e) => panic!("Bad script: {}", e)
	};

	// Taking delta time into consideration
	let new_pos = ((new_pos - obj.pos) * get_delta_time()) + obj.pos;

	if new_pos != vec2(999999., 999999.) {
		obj.update(new_pos);
		obj.try_move(obj.target);
	}
}