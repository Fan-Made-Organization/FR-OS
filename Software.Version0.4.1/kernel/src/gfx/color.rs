//! Colores: 0x00RRGGBB.
pub type Color = u32;

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub const fn channels(c: Color) -> (u8, u8, u8) {
    ((c >> 16) as u8, (c >> 8) as u8, c as u8)
}

/// Mezcla `fg` sobre `bg`. alpha 0 = solo bg, 255 = solo fg.
pub fn blend(bg: Color, fg: Color, alpha: u8) -> Color {
    let (br, bgc, bb) = channels(bg);
    let (fr, fgc, fb) = channels(fg);
    let a = alpha as u32;
    let inv = 255 - a;
    let mix = |b: u8, f: u8| ((b as u32 * inv + f as u32 * a) / 255) as u8;
    rgb(mix(br, fr), mix(bgc, fgc), mix(bb, fb))
}
