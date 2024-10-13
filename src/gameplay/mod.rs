use player::Player;
use macroquad::prelude::*;

mod player;

pub async fn gameplay() {
	let mut player = Player::new();

	loop {
		clear_background(RED);


        set_camera(&Camera2D {
			zoom: vec2(1. / camera_scale(), screen_width() / screen_height() / camera_scale()),
            target: player.pos,
            ..Default::default()
        });

		player.update();
        draw_circle(player.pos.x, player.pos.y, 15.0, YELLOW); // Player

		draw_circle(0.0, 0.0, 15.0, GREEN); // Test Object

		set_default_camera();
 
		draw_text("Test", 32.0, 64.0, camera_scale() / 10., BLACK); // Test UI

		next_frame().await;
	}
}

fn camera_scale() -> f32 {
	return screen_width() / screen_height() * 512.
}