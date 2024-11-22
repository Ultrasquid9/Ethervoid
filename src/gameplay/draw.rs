use image::DynamicImage;
use macroquad::{camera::{set_camera, set_default_camera, Camera2D}, color::{Color, RED}, math::vec2, shapes::draw_circle, texture::Texture2D, window::{clear_background, screen_height, screen_width}};
use render::draw_tilemap;
use stecs::prelude::*;

use crate::utils::{camera_scale, resources::textures::access_image};

use super::ecs::World;

pub mod process;
pub mod render;

const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

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

	for (sprite, obj) in query!([world.player], (&mut sprite, &obj)) {
		sprite.update(*obj);
		sprite.render();
	}

	for obj in query!([world.player, world.enemies], (&obj)) {
		draw_circle(obj.pos.x, obj.pos.y, obj.size, RED);
	}

	set_default_camera();
}

/// Transforms a `DynamicImage` into a `Texture2D`
pub fn to_texture(img: DynamicImage) -> Texture2D {
	let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_bytes());
	texture.set_filter(macroquad::texture::FilterMode::Nearest);
	return texture
}
