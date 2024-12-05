use std::cmp::Ordering;
use process::to_texture;
use stecs::prelude::*;

use macroquad::{
	window::{
		clear_background, 
		screen_height, 
		screen_width
	},
	camera::{
		set_camera, 
		set_default_camera, 
		Camera2D
	}, 
	color::{
		BLACK, 
		RED,
		Color
	}, 
	math::vec2, 
	shapes::draw_line, 
	text::draw_text
};

use crate::utils::{
	resources::{
		maps::access_map, 
		textures::access_image
	},
	camera_scale
};

use super::{
	combat::AttackType, 
	ecs::{sprite::Sprite, World}
};

use render::{
	draw_bar, 
	draw_tilemap
};

pub mod process;
pub mod render;

pub const SCREEN_SCALE: f32 = 3.; // TODO: make configurable

pub async fn draw(world: &mut World) {
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
	for bar in access_map(&world.current_map).doors {
		draw_bar(&bar.to_barrier());
	}

	// Handling sprites
	let mut sprites: Vec<&mut Sprite> = Vec::new();
	for (sprite, obj) in query!([world.player, world.enemies, world.npcs, world.attacks], (&mut sprite, &obj)) {
		if world.hitstop <= 0. { sprite.update(*obj) }
		sprites.push(sprite);
	}
	sprites.sort_by(|x, y| {
		if x.obj.pos.y > y.obj.pos.y {
			Ordering::Greater
		} else if x.obj.pos.y < y.obj.pos.y {
			Ordering::Less
		} else {
			Ordering::Equal
		}
	});
	for sprite in sprites {
		sprite.render().await
	}

	for (atk_type, obj) in query!(world.attacks, (&attack_type, &obj)) {
		if let AttackType::Hitscan = atk_type {
			draw_line(
				obj.pos.x, 
				obj.pos.y, 
				obj.target.x, 
				obj.target.y, 
				obj.size, 
				RED
			);
		}
	}

	set_default_camera();

	 
	// Drawing a temporary UI
	draw_text(&format!("{}", world.player.health[0].hp), 32.0, 64.0, camera_scale() / 10., BLACK);
}
