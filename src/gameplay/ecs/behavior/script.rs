use std::error::Error;
use rhai::Dynamic;
use macroquad::math::Vec2;

use crate::{
	cores::script::Script, gameplay::{
		combat::AttackStructOf, 
		ecs::{obj::Obj, sprite::Sprite}
	}, utils::get_delta_time
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
	sprite: &mut Sprite,
	attacks: &mut AttackStructOf<VecFamily>,
	current_map: &str
) -> Result<bool, Box<dyn Error + Send + Sync>> {
	// Values available in the scope
	script.scope
		.push("attacks", Vec::<Dynamic>::new())
		.push("self_current_anim", String::new())
		.push_constant("player_pos", obj_player.pos)
		.push_constant("self_pos", obj.pos);

	// Executing the script
	let new_pos = script.engine.eval_ast_with_scope::<Vec2>(&mut script.scope, &script.script)?;

	// Getting attacks out of the scope
	let new_attacks = script.scope
		.remove::<Vec<Dynamic>>("attacks")
		.expect("Attacks not found");
	for i in new_attacks {
		attacks.insert( i.clone_cast() );
	}

	// Getting the new animation from the scope
	let new_anim = script.scope.remove::<String>("self_current_anim");
	if new_anim.is_some() && new_anim.as_ref().unwrap().len() > 0 {
		sprite.set_new_anim(new_anim.unwrap())?;
	}

	// Removing constants from the scope, to prevent its length from growing over time 
	let _ = script.scope.remove::<Dynamic>("player_pos");
	let _ = script.scope.remove::<Dynamic>("self_pos");

	// Taking delta time into consideration
	let new_pos = ((new_pos - obj.pos) * get_delta_time()) + obj.pos;

	if new_pos == Vec2::new(999999., 999999.) {
		sprite.set_default_anim();
		script.scope.clear();
		return Ok(true)
	}

	obj.update(new_pos);
	obj.try_move(new_pos, current_map);

	if obj.pos != obj.target {
		sprite.set_default_anim();
		script.scope.clear();
		Ok(true)
	} else {
		Ok(false)
	}
}
