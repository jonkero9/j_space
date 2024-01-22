use std::{collections::HashMap, i32, time::Instant};

use game_color::COLORS;
use macroquad::{miniquad::window::order_quit, prelude::*};
use model::{
    star_system::StarSystem,
    vectors::Vector2DI,
};
use u_gen::factory;
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

    let mut uni_win = UniWindow::new();

    loop {
        uni_win.handle_map_movement();
        uni_win.handle_zoom();

        // set number of sectors
        let n_sectors = Vector2DI {
            x: screen_width() as i32 / uni_win.sec_size as i32,
            y: screen_height() as i32 / uni_win.sec_size as i32,
        };

        // generate star map
        let star_map: HashMap<u64, StarSystem> = factory::new_universe(
            Vector2DI {
                x: uni_win.global_pos.x as i32,
                y: uni_win.global_pos.y as i32,
            },
            n_sectors,
        );

        //Start Drawing
        clear_background(COLORS.bg);
        uni_win.handle_draw(n_sectors, star_map);
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
