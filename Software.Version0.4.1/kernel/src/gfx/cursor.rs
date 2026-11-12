//! Sprite del cursor del raton (flecha).
use super::color::{rgb, Color};

pub const WIDTH: i32 = 12;
pub const HEIGHT: i32 = 19;

// X = borde negro, . = relleno blanco
const SPRITE: [&str; 19] = [
    "X           ",
    "XX          ",
    "X.X         ",
    "X..X        ",
    "X...X       ",
    "X....X      ",
    "X.....X     ",
    "X......X    ",
    "X.......X   ",
    "X........X  ",
    "X.....XXXXX ",
    "X..X..X     ",
    "X.X X..X    ",
    "XX  X..X    ",
    "X    X..X   ",
    "     X..X   ",
    "      X..X  ",
    "      X..X  ",
    "       XX   ",
];

pub fn pixel(col: i32, row: i32) -> Option<Color> {
    let line = SPRITE.get(row as usize)?;
    match line.as_bytes().get(col as usize)? {
        b'X' => Some(rgb(0, 0, 0)),
        b'.' => Some(rgb(255, 255, 255)),
        _ => None,
    }
}
