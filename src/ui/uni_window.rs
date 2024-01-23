use std::collections::HashMap;

use jonk_utils::cantor_hash;
use macroquad::{
    color::{Color, WHITE},
    input::{is_key_down, is_key_pressed, mouse_position},
    miniquad::KeyCode,
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    model::{
        star_system::StarSystem,
        vectors::{Vector2DF, Vector2DI},
    },
    u_gen::factory,
};

use super::{draw_lines_in_window, get_n_sectors};

#[derive(Debug)]
pub struct UniWindow {
    pub sec_size: f32,
    pub global_pos: Vector2DF,
    pub show_debug: bool,
}

impl Default for UniWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl UniWindow {
    pub fn new() -> UniWindow {
        UniWindow {
            sec_size: 16.,
            global_pos: Vector2DF { x: 0., y: 0. },
            show_debug: false,
        }
    }

    pub fn handle_key_press(&mut self) {
        if is_key_pressed(KeyCode::Backslash) {
            self.show_debug = !self.show_debug;
        }
    }

    pub fn handle_map_movement(&mut self) {
        let key_sens = (16. * get_frame_time()) / (self.sec_size / 8.);
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

    pub fn handle_draw(&self, n_sectors: Vector2DI, star_map: &HashMap<u64, StarSystem>) {
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
        self.draw_debug_lines(&n_sectors);
    }

    fn draw_debug_lines(&self, n_sectors: &Vector2DI) {
        if self.show_debug {
            for y in 0..n_sectors.y {
                draw_line(
                    0.,
                    y as f32 * self.sec_size,
                    screen_width(),
                    y as f32 * self.sec_size,
                    1.,
                    WHITE,
                );
            }
            for x in 0..n_sectors.x {
                draw_line(
                    x as f32 * self.sec_size,
                    0.,
                    x as f32 * self.sec_size,
                    screen_height(),
                    1.,
                    WHITE,
                );
            }
        }
    }

    pub fn handle_mouse_hover(&self, star_map: &HashMap<u64, StarSystem>) {
        let m_pos = mouse_position();
        if let Some(star) = star_map.get(&cantor_hash(
            self.global_pos.x as i32 + (m_pos.0 / self.sec_size) as i32,
            self.global_pos.y as i32 + (m_pos.1 / self.sec_size) as i32,
        )) {
            draw_lines_in_window(
                2,
                (10., 100.),
                vec![
                    format!("Star {}, {}", star.location.x, star.location.y),
                    format!("Color: {:?}", star.star_color),
                    format!("Radius: {}", star.radius),
                    format!("Surface Temp: {}", star.surface_temp),
                    format!("Luminosity: {}", star.luminosity),
                    format!("Mass: {}", star.mass),
                    format!("Radius: {}", star.radius),
                    format!("Num of Planets: {}", star.planets.len()),
                ],
            );
        }
    }

    pub fn handle_map(&mut self) {
        self.handle_key_press();
        self.handle_map_movement();
        self.handle_zoom();
        let n_sectors = get_n_sectors(screen_width(), screen_height(), self.sec_size);
        let star_map: HashMap<u64, StarSystem> = factory::new_universe(
            Vector2DI {
                x: self.global_pos.x as i32,
                y: self.global_pos.y as i32,
            },
            n_sectors,
        );
        self.handle_draw(n_sectors, &star_map);
        self.handle_mouse_hover(&star_map);
    }
}
