use macroquad::math::Vec2;
use rhai::Dynamic;

use crate::{
	gameplay::{
		combat::AttackStructOf, 
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
