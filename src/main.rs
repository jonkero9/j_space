use macroquad::{miniquad::window::order_quit, prelude::*};
use std::time::Instant;
use ui::{draw_lines, uni_window::UniWindow};

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
        let run_time = Instant::now();
        clear_background(BLACK);

        uni_win.handle_map();

        // Draw Debug info
        draw_lines(
            (10.0, 10.0),
            vec![
                format!(
                    "fps {} pos: {:.2}, {:.2}",
                    get_fps(),
                    uni_win.global_pos.x,
                    uni_win.global_pos.y
                ),
                format!("runtime {:.4}", run_time.elapsed().as_secs_f32()),
            ],
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
