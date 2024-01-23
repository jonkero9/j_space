use macroquad::color::Color;

use crate::model::star_system::StarColor;

#[derive(Debug)]
pub struct Gamecolors {
    pub blue: Color,
    pub green: Color,
    pub white: Color,
    pub yellow: Color,
    pub orange: Color,
    pub red: Color,
    pub bg: Color,
}

impl From<StarColor> for Color {
    fn from(value: StarColor) -> Self {
        match value {
            StarColor::Red => COLORS.red,
            StarColor::Orange => COLORS.orange,
            StarColor::Yellow => COLORS.yellow,
            StarColor::White => COLORS.white,
            StarColor::Blue => COLORS.blue,
        }
    }
}

pub static COLORS: Gamecolors = Gamecolors {
    blue: Color {
        r: 137. / 255.,
        g: 180. / 255.,
        b: 250. / 255.,
        a: 1.,
    },
    white: Color {
        r: 186. / 255.,
        g: 194. / 255.,
        b: 222. / 255.,
        a: 1.,
    },
    yellow: Color {
        r: 249. / 255.,
        g: 226. / 255.,
        b: 175. / 255.,
        a: 1.,
    },
    orange: Color {
        r: 243. / 255.,
        b: 181. / 255.,
        g: 139. / 255.,
        a: 1.,
    },
    red: Color {
        r: 243. / 255.,
        g: 139. / 255.,
        b: 168. / 255.,
        a: 1.,
    },
    bg: Color {
        r: 30. / 255.,
        g: 30. / 255.,
        b: 46. / 255.,
        a: 1.,
    },
    green: Color {
        r: 167. / 255.,
        g: 227. / 255.,
        b: 161. / 255.,
        a: 1.,
    },
};
