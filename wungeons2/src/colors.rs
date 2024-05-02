pub const BG_COLOR: (u8, u8, u8, u8) = (78, 54, 42, 255);
pub const REVEALED_BG_COLOR: (u8, u8, u8, u8) = (61, 43, 31, 255);
pub const WHITE: (u8, u8, u8, u8) = (255, 255, 255, 255);
pub const BLACK: (u8, u8, u8, u8) = (0, 0, 0, 255);
pub const GOLD: (u8, u8, u8, u8) = (218, 145, 1, 255);
pub const SILVER: (u8, u8, u8, u8) = (191, 191, 191, 255);
pub const REVEALED_FOG_COLOR: (u8, u8, u8, u8) = (75, 75, 75, 255);
pub const FOG_BG_COLOR: (u8, u8, u8, u8) = (65, 65, 65, 255);


pub const WALL_COLOR: (u8, u8, u8, u8) = (50, 50, 50, 255);
pub const WALL_BG_COLOR: (u8, u8, u8, u8) = (40, 40, 40, 255);

pub fn darken_color(color: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
    let div = 2;
    (color.0 / div, color.3 / div, color.3 / div, color.3)
}
