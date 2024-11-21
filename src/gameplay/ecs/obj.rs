use macroquad::math::Vec2;

pub enum Axis {
	Positive,
	Negative,
	None
}

pub struct Obj {
	pub pos: Vec2,
	pub target: Vec2,

	pub axis_horizontal: Axis,
	pub axis_vertical: Axis,

	pub size: f32
}

impl Obj {
	pub fn new(pos: Vec2, size: f32) -> Self {
		Self {
			pos,
			target: pos,

			axis_horizontal: Axis::None,
			axis_vertical: Axis::None,

			size
		}
	}
}
