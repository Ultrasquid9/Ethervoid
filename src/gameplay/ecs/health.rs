use crate::utils::get_delta_time;

pub struct Health{
	pub hp: f32,
	i_frames: f32
}

impl Health {
	pub fn new(hp: f32) -> Self {
		Self {
			hp,
			i_frames: 0.
		}
	}

	pub fn update(&mut self) {
		if self.i_frames > 0. {
			self.i_frames -= get_delta_time()
		}
	}

	pub fn damage(&mut self, damage: f32) {
		if self.i_frames <= 0. {
			self.hp -= damage;
			self.i_frames = 10.;
		}
	}

	pub fn should_kill(&self) -> bool {
		self.hp <= 0.
	}
}
