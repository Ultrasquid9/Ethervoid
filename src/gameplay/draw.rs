use macroquad::prelude::*;
use std::cmp::Ordering;
use stecs::prelude::*;

use crate::{
	gameplay::draw::{process::to_texture, render::render_line},
	menu::average_screen_size,
	utils::{
		camera_scale,
		resources::{maps::access_map, textures::access_image},
	},
};

use super::{
	Gameplay,
	combat::AttackType,
	ecs::{behavior::Behavior, sprite::Sprite},
	paused::Paused,
};

use render::{draw_bar, draw_map, render_text, render_texture};

pub mod process;
pub mod render;
pub mod ui;

pub async fn draw(gameplay: &mut Gameplay) {
	// Draws the background
	clear_background(Color::from_rgba(46, 34, 47, 255));

	set_camera(&Camera2D {
		zoom: vec2(
			1. / camera_scale() as f32,
			screen_width() / screen_height() / camera_scale() as f32,
		),
		target: gameplay
			.world
			.player
			.obj
			.first()
			.expect("Player should exist")
			.pos
			.as_vec2(),
		..Default::default()
	});

	draw_map(access_map(&gameplay.current_map)).await;

	for wall in &access_map(&gameplay.current_map).walls {
		for bar in wall {
			draw_bar(bar);
		}
	}
	for door in &access_map(&gameplay.current_map).doors {
		draw_bar(&door.to_barrier());
	}

	render_sprites(gameplay).await;

	for (atk_type, sprite) in query!(gameplay.world.attacks, (&atk_type, &mut sprite)) {
		if let AttackType::Hitscan = atk_type {
			render_line(sprite).await;
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
			err_height += 32.;
		}
	}

	let ui = to_texture(access_image("default:ui/hp"));
	draw_texture_ex(
		&ui,
		0.,
		0.,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(ui.width(), ui.height()) * (average_screen_size() / 300.)),
			..Default::default()
		},
	);

	for (ui, health) in query!(gameplay.world.player, (&ui, &health)) {
		ui.draw_hp(health);
		ui.draw_temp(miniquad::date::now().sin().abs() * 100.); // TODO: Temperature system 
	}
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
		if gameplay.paused == Paused::None {
			sprite.update(*obj);
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
	for sprite in &mut sprites {
		let (texture, pos, params) = sprite.as_render_params();
		render_texture(&texture, pos, params).await;
	}
}
