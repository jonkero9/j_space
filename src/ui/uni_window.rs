use std::collections::HashMap;

use macroquad::{input::is_key_down, miniquad::KeyCode, time::get_frame_time, shapes::draw_circle, color::Color};

use crate::model::{vectors::{Vector2DF, Vector2DI}, star_system::StarSystem};

#[derive(Debug)]
pub struct UniWindow {
    pub sec_size: f32,
    pub global_pos: Vector2DF,
}

impl UniWindow {
    pub fn new() -> UniWindow {
        UniWindow {
            sec_size: 16.,
            global_pos: Vector2DF { x: 0., y: 0. },
        }
    }

    pub fn handle_map_movement(&mut self) {
        let key_sens = (4. * get_frame_time()) / (self.sec_size / 16.);
        //input handle
        if is_key_down(KeyCode::W) {
            self.global_pos.y -= key_sens;
        }
        if is_key_down(KeyCode::A) {
            self.global_pos.x -= key_sens;
        }
        if is_key_down(KeyCode::S) {
            self.global_pos.y += key_sens;
        }
        if is_key_down(KeyCode::D) {
            self.global_pos.x += key_sens;
        }
    }

    pub fn handle_zoom(&mut self) {
        let zoom_sens = 2. * get_frame_time();
        if is_key_down(KeyCode::E) {
            self.sec_size += zoom_sens;
        }
        if is_key_down(KeyCode::Q) && self.sec_size > 16. {
            self.sec_size -= zoom_sens;
        }
    }

    pub fn handle_draw(&self, n_sectors: Vector2DI, star_map: HashMap<u64, StarSystem>) {
        for y in 0..n_sectors.y {
            for x in 0..n_sectors.x {
                let global_sec = Vector2DI {
                    x: self.global_pos.x as i32 + x,
                    y: self.global_pos.y as i32 + y,
                };
                let hash_key = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
                if let Some(star) = star_map.get(&hash_key) {
                    let sec_to_screen = Vector2DF {
                        x: x as f32 * self.sec_size,
                        y: y as f32 * self.sec_size,
                    };
                    draw_circle(
                        sec_to_screen.x + (self.sec_size / 2.),
                        sec_to_screen.y + (self.sec_size / 2.),
                        (star.radius / 2000.) * (self.sec_size / 2.),
                        Color::from(star.star_color),
                    );
                }
            }
        }
    }
}
