use macroquad::window::next_frame;

use crate::State;

pub mod ecs;
pub mod player;

pub async fn gameplay() -> State {

	loop {
		next_frame().await
	}
}