use std::sync::RwLock;

use macroquad::math::Vec2;
use once_cell::sync::Lazy;
use raylite::{cast, cast_wide, Barrier, Ray};

use crate::utils::{tuple_to_vec2, vec2_to_tuple};

use super::{cores::map::Map, draw::texturedobj::EntityTexture, player::Axis};

// For keeping track of the recursion in `try_move`
static DEPTH: Lazy<RwLock<u8>> = Lazy::new(|| RwLock::new(0));

/// Trait for an object that has a size and can be moved
pub trait MovableObj {
	fn get_size(&self) -> &f32;
	fn get_pos(&self) -> Vec2;
	fn edit_pos(&mut self) -> &mut Vec2;

	/// Attempts to move the object to the provided Vec2
	fn try_move(&mut self, target: Vec2, map: &Map) {
		let mut barriers = create_barriers(&map.points);
		for i in &map.doors {
			barriers.push(i.to_barrier())
		}

		let old_pos = self.get_pos();
		let mut try_slope_movement = false;

		match cast_wide(
			&Ray {
				position: (self.get_pos().x, self.get_pos().y),
				end_position: (target.x, self.get_pos().y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.edit_pos().x = target.x
		}
	
		match cast_wide(
			&Ray {
				position: (self.get_pos().x, self.get_pos().y),
				end_position: (self.get_pos().x, target.y)
			}, 
			&barriers
		) {
			Ok(_) => try_slope_movement = true,
			_ => self.edit_pos().y = target.y
		}

		// Everything beyond this point is for handling slopes
		if !try_slope_movement { return }

		// Checking recursion
		if *DEPTH.read().unwrap() > 1 {
			*DEPTH.write().unwrap() = 0;
			return 
		} else {
			*DEPTH.write().unwrap() += 1
		}

		let mut wall_to_check = Barrier {
			// Rust assumes that this variable could possibly be uninitialized,
			// so I have to set a burner value that is never read. 
			positions: ((0., 0.), (0., 0.)) 
		};

		for i in barriers {
			match cast(
				&Ray {
					position: (self.get_pos().x, self.get_pos().y),
					end_position: (target.x, target.y)
				}, 
				&i
			) {
				Ok(_) => (),
				_ => wall_to_check = i
			}
		}

		if wall_to_check.positions.0.0 != wall_to_check.positions.1.0
		&& wall_to_check.positions.0.1 != wall_to_check.positions.1.1 {
			return;
		}
		
		let angle = tuple_to_vec2(wall_to_check.positions.0).angle_between(tuple_to_vec2(wall_to_check.positions.1));

		let new_pos = Vec2::new(
			old_pos.distance(target) * angle.cos(), 
			old_pos.distance(target) * angle.cos()
		);

		if (old_pos + new_pos).distance(target) > old_pos.distance(target) {
			self.try_move(old_pos + new_pos, map);
		} else {
			self.try_move(old_pos - new_pos, map);
		}
	}

	/// Checks if the object is touching another object
	fn is_touching(&self, other: &dyn MovableObj,) -> bool {
		if self.get_pos().distance(other.get_pos()) <= *self.get_size() + *other.get_size() {
			return true;
		} else {
			return false;
		}
	}
}

/// Data used by all entities, including both the player and enemies
#[derive(PartialEq)]
pub struct Entity {
	pub i_frames: u8,
	pub stunned: u8,
	pub size: f32,
	health: isize,

	pub dir_horizontal: Axis,
	pub dir_vertical: Axis,
	pos: Vec2,

	pub texture: EntityTexture
}

impl Entity {
	/// Returns the x position
	pub fn x(&self) -> f32 {
		return self.pos.x;
	}

	/// Returns the y position
	pub fn y(&self) -> f32 {
		return self.pos.y;
	}

	/// Gets the health
	pub fn get_health(&self) -> isize {
		return self.health
	}

	/// Updates the current directions based upon an old and new position 
	pub fn update_axis(&mut self, new_pos: &Vec2) {
		if self.pos == *new_pos {
			self.texture.moving = false;
			return
		} else {
			self.texture.moving = true;
		}

		let new_axis = get_axis(self.pos, *new_pos);

		self.dir_horizontal = new_axis.0;
		self.dir_vertical = new_axis.1;
	}

	/// Creates a new Entity
	pub fn new(pos: Vec2, size: f32, health: isize, texture: EntityTexture) -> Self {
		return Entity {
			i_frames: 0,
			stunned: 0,
			size,
			health,

			dir_horizontal: Axis::None,
			dir_vertical: Axis::None,
			pos,

			texture
		}
	}

	pub fn try_damage(&mut self, damage: isize) {
		if self.i_frames == 0 {
			self.health -= damage;
			self.i_frames = 12;
		}
	}

	/// Checks if the entity is dead
	pub fn should_kill(&self) -> bool {
		if self.health <= 0 {
			return true
		} else {
			return false
		}
	}
}

// Allows the entity to be moved 
impl MovableObj for Entity {
	fn get_size(&self) -> &f32 {
		&self.size
	}

	fn get_pos(&self) -> Vec2 {
		return self.pos
	}

	fn edit_pos(&mut self) -> &mut Vec2 {
		&mut self.pos
	}
}

pub fn get_axis(pos: Vec2, target: Vec2) -> (Axis, Axis) {
	let x_val = (pos.x - target.x) / pos.distance(target);
	let y_val = (pos.y - target.y) / pos.distance(target);

	let mut to_return = (Axis::None, Axis::None);

	match x_val.round() as i8 {
		-1 => to_return.0 = Axis::Positive,
		0 => to_return.0 = Axis::None,
		1 => to_return.0 = Axis::Negative,
		_ => ()
	}

	match y_val.round() as i8 {
		-1 => to_return.1 = Axis::Positive,
		0 => to_return.1 = Axis::None,
		1 => to_return.1 = Axis::Negative,
		_ => ()
	}

	return to_return
}

fn create_barriers(map: &Vec<Vec2>) -> Vec<Barrier> {
	let mut barriers: Vec<Barrier> = Vec::new();

	for i in 0..map.len() {
		match map.get(i + 1) {
			Some(_) => barriers.push(Barrier {
				positions: (vec2_to_tuple(map.get(i).unwrap()), vec2_to_tuple(map.get(i + 1).unwrap()))
			}),
			None => barriers.push(Barrier {
				positions: (vec2_to_tuple(map.get(i).unwrap()), (vec2_to_tuple(map.get(0).unwrap())))
			})
		}
	}

	return barriers;
}
