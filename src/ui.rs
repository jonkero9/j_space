use std::i32;

use crate::{game_color::COLORS, model::vectors::Vector2DI};
use macroquad::{
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
};
pub mod uni_window;

const FONT_SIZE: f32 = 32.;
const MARGIN_SIZE: f32 = 16.;

pub fn get_n_sectors(screen_x: f32, screen_y: f32, sec_size: f32) -> Vector2DI {
    Vector2DI {
        x: screen_x as i32 / sec_size as i32,
        y: screen_y as i32 / sec_size as i32,
    }
}

pub fn draw_lines(pos: (f32, f32), lines: Vec<String>) {
    let longest_string = lines.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap();
    let text_measure = measure_text(&longest_string, None, FONT_SIZE as u16, 1.);
    draw_rectangle(
        pos.0,
        pos.1,
        text_measure.width + (2. * MARGIN_SIZE),
        ((text_measure.height * lines.len() as f32) + text_measure.height) + (2. * MARGIN_SIZE),
        COLORS.bg,
    );
    for (i, ele) in lines.iter().enumerate() {
        draw_text(
            &ele,
            pos.0 + MARGIN_SIZE,
            MARGIN_SIZE + ((pos.1 + text_measure.height) * (i as f32 + 1.)),
            FONT_SIZE,
            COLORS.white,
        );
    }
}
