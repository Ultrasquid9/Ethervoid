use macroquad::math::Vec2;

/// Data used by all entities, including both the player and enemies
pub struct Entity {
	pos: Vec2,

	pub size: f32,
	pub health: isize
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

	/// Returns the position
	pub fn get_pos(&self) -> Vec2 {
		return self.pos;
	}

	/// Creates a new Entity
	pub fn new(pos: Vec2, size: f32, health: isize) -> Self {
		return Entity {
			pos,
			size,
			health
		}
	}

	/// Tries to move the entity to the provided Vec2
	pub fn try_move(&mut self, new_pos: Vec2) {
		self.pos = new_pos;
	}
}