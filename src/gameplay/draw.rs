use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{Color, RED}, math::vec2, shapes::draw_circle, window::{clear_background, screen_height, screen_width}};
use stecs::prelude::*;

use crate::utils::camera_scale;

use super::ecs::World;

pub async fn draw<'a>(world: &World<'a>) {
	// Draws the background
	clear_background(Color::from_rgba(
		46, 
		34, 
		47, 
		255
	)); 

	set_camera(&Camera2D {
		zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
		target: world.player.obj.first().unwrap().pos,
		..Default::default()
	});

	for obj in query!([world.player, world.enemies], (&obj)) {
		draw_circle(obj.pos.x, obj.pos.y, obj.size, RED);
	}

	set_default_camera();
}
