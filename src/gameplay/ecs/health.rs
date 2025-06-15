use crate::utils::smart_time;

pub struct Health {
	pub hp: f64,
	pub max: f64,
	i_frames: f64,
}

impl Health {
	pub fn new(hp: f64) -> Self {
		Self {
			hp,
			max: hp,
			i_frames: 0.,
		}
	}

	pub fn update(&mut self) {
		if self.i_frames > 0. {
			self.i_frames -= smart_time();
		}
	}

	pub fn damage(&mut self, damage: f64) {
		if self.i_frames <= 0. {
			self.hp -= damage;
			self.i_frames = 10.;
		}
	}

	pub fn should_kill(&self) -> bool {
		self.hp <= 0.
	}
}
