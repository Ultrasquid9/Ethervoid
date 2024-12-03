use macroquad::math::Vec2;
use rhai::Dynamic;

use crate::{
	gameplay::{
		combat::{
			Attack, 
			AttackStructOf, 
			Owner
		}, 
		ecs::obj::Obj
	}, 
	utils::get_delta_time,
	cores::script::Script
};

use stecs::{
	storage::vec::VecFamily,
	prelude::*
};

/// Reads a script. Returns true if the script has finished, or the Obj could not move
pub fn script_behavior(
	script: &mut Script, 
	obj: &mut Obj, 
	obj_player: &Obj,
	attacks: &mut AttackStructOf<VecFamily>,
	current_map: &str
) -> bool {
	// Values available in the scope
	script.scope
		.push("attacks", Vec::<Dynamic>::new())
		.push_constant("player_pos", obj_player.pos)
		.push_constant("self_pos", obj.pos);

	// Values needed for the script, but not exposed to it
	let obj_clone = *obj;

	// Registerring functions for the script
	script.engine
		// Functions for creating attacks
		.register_fn("new_physical", move |damage: f32, size, target: Vec2,| Attack::new_physical(
			Obj::new(obj_clone.pos, target, size),
			damage, 
			Owner::Enemy,
			"default:attacks/dash"
		))
		.register_fn("new_burst", move |damage: f32, size| Attack::new_burst(
			Obj::new(obj_clone.pos, obj_clone.pos, size), 
			damage, 
			Owner::Enemy,
			"default:attacks/burst"
		))
		.register_fn("new_projectile", move |damage: f32, target: Vec2| Attack::new_projectile(
			Obj::new(obj_clone.pos, target, 10.),
			damage, 
			Owner::Enemy,
			"default:attacks/projectile-enemy"
		))
		.register_fn("new_hitscan", move |damage: f32, target: Vec2| Attack::new_hitscan(
			Obj::new(obj_clone.pos, target, 6.),
			damage, 
			Owner::Enemy
		));

	// Executing the script
	let new_pos = match script.engine.eval_with_scope::<Vec2>(&mut script.scope, &script.script) {
		Ok(new_pos) => new_pos,
		Err(e) => panic!("Bad script: {}", e)
	};

	// Getting attacks out of the scope
	let new_attacks = script.scope
		.remove::<Vec<Dynamic>>("attacks")
		.expect("Attacks not found");
	for i in new_attacks {
		attacks.insert( i.clone_cast() );
	}

	// Removing constants from the scope, to prevent its length from growing exponentially 
	let _ = script.scope.remove::<Dynamic>("player_pos");
	let _ = script.scope.remove::<Dynamic>("self_pos");

	// Taking delta time into consideration
	let new_pos = ((new_pos - obj.pos) * get_delta_time()) + obj.pos;

	if new_pos != Vec2::new(999999., 999999.) {
		obj.update(new_pos);
		obj.try_move(new_pos, current_map);
	} else {
		script.scope.clear();
		return true
	}

	if obj.pos != obj.target {
		script.scope.clear();
		true
	} else {
		false
	}
}
