//! Graficos: framebuffer, fuente y consola.
pub mod console;
pub mod font;
pub mod framebuffer;

pub type Color = (u8, u8, u8);

pub const BG: Color = (18, 18, 24);
pub const FG: Color = (200, 220, 200);
pub const ACCENT: Color = (90, 180, 255);
pub const ERROR: Color = (255, 110, 110);
