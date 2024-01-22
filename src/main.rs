use std::{collections::HashMap, i32, time::Instant};

use game_color::COLORS;
use macroquad::{miniquad::window::order_quit, prelude::*};
use model::{
    star_system::StarSystem,
    vectors::{Vector2DF, Vector2DI},
};
use u_gen::factory;

pub mod game_color;
pub mod model;
pub mod u_gen;

#[macroquad::main("J_space")]
async fn main() {
    // set fps value
    let min_f_time = 1. / 60.;
    // timer for fps
    let mut fps_timer = Instant::now();

    let mut sec_size = 16.;
    let mut global_pos = Vector2DF { x: 0., y: 0. };

    loop {
        // set number of sectors
        let n_sectors = Vector2DI {
            x: screen_width() as i32 / sec_size as i32,
            y: screen_height() as i32 / sec_size as i32,
        };
        // generate star map
        let star_map: HashMap<u64, StarSystem> = factory::new_universe(
            Vector2DI {
                x: global_pos.x as i32,
                y: global_pos.y as i32,
            },
            n_sectors,
        );

        let key_sens = (6. * get_frame_time()) / (sec_size / 16.);
        //input handle
        if is_key_down(KeyCode::W) {
            global_pos.y -= key_sens;
        }
        if is_key_down(KeyCode::A) {
            global_pos.x -= key_sens;
        }
        if is_key_down(KeyCode::S) {
            global_pos.y += key_sens;
        }
        if is_key_down(KeyCode::D) {
            global_pos.x += key_sens;
        }
        // handle quit key
        if is_key_down(KeyCode::Escape) {
            order_quit();
        }

        let zoom_sens = 2. * get_frame_time();
        if is_key_down(KeyCode::E) {
            sec_size += zoom_sens;
        }
        if is_key_down(KeyCode::Q) && sec_size > 16. {
            sec_size -= zoom_sens;
        }

        //Start Drawing
        clear_background(COLORS.bg);
        for y in 0..n_sectors.y {
            for x in 0..n_sectors.x {
                let global_sec = Vector2DI {
                    x: global_pos.x as i32 + x,
                    y: global_pos.y as i32 + y,
                };
                let hash_key = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
                if let Some(star) = star_map.get(&hash_key) {
                    let sec_to_screen = Vector2DF {
                        x: x as f32 * sec_size,
                        y: y as f32 * sec_size,
                    };
                    draw_circle(
                        sec_to_screen.x + (sec_size / 2.),
                        sec_to_screen.y + (sec_size / 2.),
                        (star.radius / 2000.) * (sec_size / 2.),
                        Color::from(star.star_color),
                    );
                }
            }
        }

        draw_text(
            &format!("fps {} pos: {}, {}", get_fps(), global_pos.x, global_pos.y),
            20.,
            20.,
            32.,
            COLORS.white,
        );

        // check frame time for fps
        if min_f_time < fps_timer.elapsed().as_secs_f32() {
            fps_timer = Instant::now();
            next_frame().await
        }
    }
}
