use crate::model::vectors::Vector2DI;

pub mod uni_window;

pub fn get_n_sectors(screen_x: f32, screen_y: f32, sec_size: f32) -> Vector2DI {
    Vector2DI {
        x: screen_x as i32 / sec_size as i32,
        y: screen_y as i32 / sec_size as i32,
    }
}
