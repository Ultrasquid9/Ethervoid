use std::cmp::Ordering;
use process::to_texture;
use stecs::prelude::*;
use macroquad::prelude::*;

use crate::utils::{
	resources::{
		maps::access_map, 
		textures::access_image
	},
	camera_scale
};

use super::{
	ecs::{
		behavior::Behavior,
		sprite::Sprite, 
		World
	},
	combat::AttackType
};

use render::{
	draw_bar, 
	draw_tilemap, 
	render_text, render_texture
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

	render_sprites(world).await;

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

	// Render script errors (if any are present)
	let mut err_height = 128.;
	for behavior in query!(world.enemies, (&behavior)) {
		let Behavior::Enemy(behavior) = behavior else { continue };

		if let Some(e) = &behavior.err {
			render_text(&format!("Script err: {e}"), Vec2::new(32., err_height), RED).await;
			err_height += 32.
		}
	}
	 
	// Drawing a temporary UI
	render_text(&format!("{}", world.player.health[0].hp), Vec2::new(32., 96.), BLACK).await
}

async fn render_sprites(world: &mut World) {
	// Sorting sprites
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

	// Processing sprites
	let mut stuff = Vec::new();

	sprites.iter().for_each(|sprite| {
		stuff.push(sprite.to_render_params());
	});

	// Rendering sprites
	for (texture, pos, params) in stuff {
		render_texture(&to_texture(texture), pos, params).await
	}
}
