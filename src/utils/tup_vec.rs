use macroquad::math::*;
use raywoke::point::Point;

pub trait Tup64 {
	/// Converts the type into an ([f64](core::f64), [f64](core::f64))
	fn tup64(self: &Self) -> (f64, f64);
}

pub trait DV2 {
	/// Converts the type into a [DVec2]
	fn dvec2(self: &Self) -> DVec2;
}

impl Tup64 for DVec2 {
	fn tup64(self: &Self) -> (f64, f64) {
		(self.x, self.y)
	}
}

impl DV2 for (f64, f64) {
	fn dvec2(self: &Self) -> DVec2 {
		dvec2(self.x(), self.y())
	}
}

impl DV2 for dyn Point {
	fn dvec2(self: &Self) -> DVec2 {
		dvec2(self.x(), self.y())
	}
}
