use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::Color, math::vec2, window::{clear_background, screen_height, screen_width}};
use process::to_texture;
use render::{draw_bar, draw_tilemap};
use stecs::prelude::*;

use crate::utils::{camera_scale, resources::{maps::access_map, textures::access_image}};

use super::ecs::World;

pub mod process;
pub mod render;

pub const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

pub async fn draw<'a>(world: &mut World<'a>) {
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

	draw_tilemap(to_texture(access_image("default:tiles/grass_test"))).await;

	for bar in access_map(&world.current_map).walls {
		draw_bar(&bar);
	}

	for (sprite, obj) in query!([world.player, world.enemies], (&mut sprite, &obj)) {
		sprite.update(*obj);
		sprite.render().await;
	}

	set_default_camera();
}
