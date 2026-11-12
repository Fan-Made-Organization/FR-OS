//! Iconos pequenos de la barra, dibujados con rectangulos y sprites.
use super::theme;
use crate::gfx::{canvas::Canvas, color::Color, rect::Rect};

pub const NETWORK_W: i32 = 15;
pub const NETWORK_H: i32 = 12;
pub const BLUETOOTH_W: i32 = 9;
pub const BLUETOOTH_H: i32 = 13;
pub const BATTERY_W: i32 = 22;
pub const BATTERY_H: i32 = 11;

/// Cuatro barritas de senal.
pub fn network(canvas: &mut Canvas, x: i32, y: i32, color: Color) {
    for i in 0..4 {
        let h = 3 + 3 * i;
        canvas.fill_rect(Rect::new(x + i * 4, y + NETWORK_H - h, 3, h), color);
    }
}

const BLUETOOTH: [&str; 13] = [
    "....#....",
    "....##...",
    "....#.#..",
    "#...#..#.",
    ".#..#.#..",
    "..#.##...",
    "...##....",
    "..#.##...",
    ".#..#.#..",
    "#...#..#.",
    "....#.#..",
    "....##...",
    "....#....",
];

pub fn bluetooth(canvas: &mut Canvas, x: i32, y: i32, color: Color) {
    canvas.draw_sprite(&BLUETOOTH, x, y, color);
}

/// Bateria; `level` en porcentaje, o None si no se puede leer.
pub fn battery(canvas: &mut Canvas, x: i32, y: i32, color: Color, level: Option<u8>) {
    canvas.fill_rect(Rect::new(x, y, 20, BATTERY_H), color);
    canvas.fill_rect(Rect::new(x + 1, y + 1, 18, BATTERY_H - 2), theme::BAR_BG);
    canvas.fill_rect(Rect::new(x + 20, y + 3, 2, 5), color);
    if let Some(p) = level {
        let w = 16 * p.min(100) as i32 / 100;
        canvas.fill_rect(Rect::new(x + 2, y + 2, w, BATTERY_H - 4), color);
    }
}
