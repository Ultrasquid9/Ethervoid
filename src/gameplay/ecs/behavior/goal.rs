use macroquad::math::DVec2;
use rhai::{CallFnOptions, Dynamic};

use crate::{
	cores::goal::Goal,
	gameplay::{
		combat::AttackStructOf,
		ecs::{obj::Obj, sprite::Sprite},
	},
	utils::{error::Result, get_delta_time},
};

use stecs::{prelude::*, storage::vec::VecFamily};

impl Goal {
	pub fn update_constants(&mut self, obj_self: &Obj, obj_player: &Obj) {
		// Removing the constants if they exist, to prevent the scope from growing exponentially
		_ = self.scope.remove::<Dynamic>("player_pos");
		_ = self.scope.remove::<Dynamic>("self_pos");

		// Re-adding the constants with updated values
		self.scope
			.push_constant("pos_self", obj_self.pos)
			.push_constant("pos_player", obj_player.pos);
	}

	pub fn should_start(&mut self) -> Result<bool> {
		let result =
			self.engine
				.call_fn::<bool>(&mut self.scope, &self.script, "should_start", ())?;

		Ok(result)
	}

	pub fn should_stop(&mut self, sprite: &mut Sprite) -> Result<bool> {
		let x = self
			.engine
			.call_fn::<bool>(&mut self.scope, &self.script, "should_stop", ())?;

		if x {
			self.scope.clear();
			sprite.set_default_anim();
			Ok(true)
		} else {
			Ok(false)
		}
	}

	pub fn init(&mut self) -> Result<()> {
		self.engine.call_fn_with_options::<()>(
			CallFnOptions::new().eval_ast(false).rewind_scope(false),
			&mut self.scope,
			&self.script,
			"init",
			(),
		)?;

		Ok(())
	}

	pub fn update(
		&mut self,
		obj: &mut Obj,
		sprite: &mut Sprite,
		attacks: &mut AttackStructOf<VecFamily>,
		current_map: &str,
	) -> Result<()> {
		// Values available in the scope
		self.scope
			.push("attacks", Vec::<Dynamic>::new())
			.push("current_anim", String::new());

		// Executing the script
		let new_pos = self
			.engine
			.call_fn::<DVec2>(&mut self.scope, &self.script, "update", ())?;

		// Getting attacks out of the scope
		let new_attacks = self
			.scope
			.remove::<Vec<Dynamic>>("attacks")
			.expect("Attacks not found");
		for i in new_attacks {
			attacks.insert(i.clone_cast());
		}

		// Getting the new animation from the scope
		let new_anim = self.scope.remove::<String>("current_anim");
		if new_anim.is_some() && !new_anim.as_ref().unwrap().is_empty() {
			sprite.set_new_anim(new_anim.unwrap())?;
		}

		// Taking delta time into consideration
		let new_pos = ((new_pos - obj.pos) * get_delta_time()) + obj.pos;

		obj.update(new_pos);
		obj.try_move(new_pos, current_map);

		Ok(())
	}
}
