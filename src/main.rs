use std::{collections::HashMap, i32, time::Instant};

use game_color::COLORS;
use macroquad::prelude::*;
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

    let sec_size = 16.;
    let global_pos = Vector2DF { x: 0., y: 0. };

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

        //Start Drawing
        clear_background(COLORS.bg);
        for star in star_map.values() {
            draw_circle(
                (star.location.x as f32 * sec_size) + (sec_size / 2.),
                (star.location.y as f32 * sec_size) + (sec_size / 2.),
                (star.radius / 2000.) * (sec_size / 2.),
                Color::from(star.star_color),
            );
        }
        draw_text(&format!("{}", get_fps()), 20., 20., 32., COLORS.white);

        // check frame time for fps
        if min_f_time < fps_timer.elapsed().as_secs_f32() {
            fps_timer = Instant::now();
            next_frame().await
        }
    }
}
