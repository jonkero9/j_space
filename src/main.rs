use game_color::COLORS;
use macroquad::{miniquad::window::order_quit, prelude::*, ui::root_ui};
use std::time::Instant;
use ui::{draw_lines_in_window, uni_window::UniWindow};

pub mod game_color;
pub mod model;
pub mod u_gen;
pub mod ui;

#[macroquad::main("J_space")]
async fn main() {
    let mut theme_skin = root_ui().default_skin();
    theme_skin.label_style = root_ui()
        .style_builder()
        .text_color(COLORS.white)
        .font_size(ui::FONT_SIZE as u16)
        .build();
    theme_skin.window_style = root_ui()
        .style_builder()
        .margin(RectOffset::new(
            ui::MARGIN_SIZE,
            ui::MARGIN_SIZE,
            ui::MARGIN_SIZE,
            ui::MARGIN_SIZE,
        ))
        .color(COLORS.bg)
        .build();
    root_ui().push_skin(&theme_skin);

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
        draw_lines_in_window(
            1,
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
