use macroquad::{color::{Color, RED}, shapes::draw_circle, window::clear_background};
use stecs::prelude::*;

use super::ecs::World;

pub async fn draw<'a>(world: &World<'a>) {
	// Draws the background
	clear_background(Color::from_rgba(
		46, 
		34, 
		47, 
		255
	)); 

	for obj in query!([world.player, world.enemies], (&obj)) {
		draw_circle(obj.pos.x, obj.pos.y, obj.size, RED);
	}
}
