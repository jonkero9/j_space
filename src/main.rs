use game_color::COLORS;
use macroquad::{miniquad::window::order_quit, prelude::*};
use std::time::Instant;
use ui::uni_window::UniWindow;

pub mod game_color;
pub mod model;
pub mod u_gen;
pub mod ui;

#[macroquad::main("J_space")]
async fn main() {
    // set fps value
    let min_f_time = 1. / 60.;
    // timer for fps
    let mut fps_timer = Instant::now();
    // universe map window
    let mut uni_win = UniWindow::new();

    loop {
        clear_background(BLACK);

        uni_win.handle_map();

        // Draw Debug info
        draw_text(
            &format!(
                "fps {} pos: {}, {}",
                get_fps(),
                uni_win.global_pos.x,
                uni_win.global_pos.y
            ),
            20.,
            20.,
            32.,
            COLORS.white,
        );

        // handle quit key
        if is_key_down(KeyCode::Escape) {
            order_quit();
        }

        // check frame time for fps
        if min_f_time < fps_timer.elapsed().as_secs_f32() {
            fps_timer = Instant::now();
            next_frame().await
        }
    }
}
