use macroquad::prelude::*;
use process::to_texture;
use std::cmp::Ordering;
use stecs::prelude::*;

use crate::utils::{camera_scale, resources::maps::access_map};

use super::{
	Gameplay,
	combat::AttackType,
	ecs::{behavior::Behavior, sprite::Sprite},
};

use render::{draw_bar, draw_map, render_text, render_texture};

pub mod process;
pub mod render;

pub const SCREEN_SCALE: f64 = 3.; // TODO: make configurable

pub async fn draw(gameplay: &mut Gameplay) {
	// Draws the background
	clear_background(Color::from_rgba(46, 34, 47, 255));

	set_camera(&Camera2D {
		zoom: vec2(
			1. / camera_scale() as f32,
			screen_width() / screen_height() / camera_scale() as f32,
		),
		target: gameplay.world.player.obj.first().unwrap().pos.as_vec2(),
		..Default::default()
	});

	draw_map(access_map(&gameplay.current_map)).await;

	for wall in &access_map(&gameplay.current_map).walls {
		for bar in wall {
			draw_bar(bar);
		}
	}
	for door in access_map(&gameplay.current_map).doors.iter() {
		draw_bar(&door.to_barrier());
	}

	render_sprites(gameplay).await;

	for (atk_type, obj) in query!(gameplay.world.attacks, (&attack_type, &obj)) {
		if let AttackType::Hitscan = atk_type {
			draw_line(
				obj.pos.x as f32,
				obj.pos.y as f32,
				obj.target.x as f32,
				obj.target.y as f32,
				obj.size as f32,
				RED,
			);
		}
	}

	set_default_camera();

	// Render script errors (if any are present)
	let mut err_height = 128.;
	for behavior in query!(gameplay.world.enemies, (&behavior)) {
		let Behavior::Goal(behavior) = behavior else {
			continue;
		};

		if let Some(e) = &behavior.err {
			render_text(
				&format!("Script err: {e}"),
				DVec2::new(32., err_height),
				RED,
			)
			.await;
			err_height += 32.
		}
	}

	// Drawing a temporary UI
	render_text(
		&format!("{}", gameplay.world.player.health[0].hp),
		DVec2::new(32., 96.),
		BLACK,
	)
	.await
}

async fn render_sprites(gameplay: &mut Gameplay) {
	// Sorting sprites
	let mut sprites: Vec<&mut Sprite> = vec![];
	for (sprite, obj) in query!(
		[
			gameplay.world.player,
			gameplay.world.enemies,
			gameplay.world.npcs,
			gameplay.world.attacks
		],
		(&mut sprite, &obj)
	) {
		if gameplay.hitstop <= 0. {
			sprite.update(*obj)
		}
		sprites.push(sprite);
	}
	sprites.sort_by(|x, y| {
		if x.obj().pos.y > y.obj().pos.y {
			Ordering::Greater
		} else if x.obj().pos.y < y.obj().pos.y {
			Ordering::Less
		} else {
			Ordering::Equal
		}
	});

	// Processing sprites
	let mut futures = vec![];

	sprites.iter_mut().for_each(|sprite| {
		futures.push(sprite.to_render_params());
	});

	// Rendering sprites
	for future in futures {
		let (texture, pos, params) = future.await;
		render_texture(&to_texture(texture), pos, params).await;
	}
}
