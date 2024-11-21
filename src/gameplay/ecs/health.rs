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
}
