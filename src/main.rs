mod input;

use macroquad::prelude::*;
use crate::input::get_keycode;

#[macroquad::main("Ethervoid")]
async fn main() {
	let mut test_player = Vec2 {
		x: screen_width() / 2.0,
		y: screen_height() / 2.0
	};

    loop {
        clear_background(RED);

		if is_key_down(get_keycode("Up")) {
			test_player.y -= 1.0;
		}
		if is_key_down(get_keycode("Down")) {
			test_player.y += 1.0;
		}
		if is_key_down(get_keycode("Left")) {
			test_player.x -= 1.0;
		}
		if is_key_down(get_keycode("Right")) {
			test_player.x += 1.0;
		}

        draw_circle(test_player.x, test_player.y, 15.0, YELLOW);

        next_frame().await
    }
}
