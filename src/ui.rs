use crate::{game_color::COLORS, model::vectors::Vector2DI};
use macroquad::{
    math::i32,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
};
pub mod uni_window;

const FONT_SIZE: f32 = 32.;

pub fn get_n_sectors(screen_x: f32, screen_y: f32, sec_size: f32) -> Vector2DI {
    Vector2DI {
        x: screen_x as i32 / sec_size as i32,
        y: screen_y as i32 / sec_size as i32,
    }
}

pub fn draw_lines(pos: (f32, f32), lines: Vec<String>) {
    let joined = lines.join("\n");
    let text_measure = measure_text(&joined, None, FONT_SIZE as u16, 1.);
    draw_rectangle(
        pos.0,
        pos.1,
        text_measure.width,
        text_measure.height+ FONT_SIZE,
        COLORS.bg,
    );
    draw_text(&joined, pos.0, pos.1 + FONT_SIZE, FONT_SIZE, COLORS.white);
}
