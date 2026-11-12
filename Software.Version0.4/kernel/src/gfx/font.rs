//! Fuente bitmap (Noto Sans Mono, 16 px).
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight, RasterizedChar};

const WEIGHT: FontWeight = FontWeight::Regular;
const HEIGHT: RasterHeight = RasterHeight::Size16;

pub const CHAR_HEIGHT: i32 = 16;

/// Glifo de un caracter; si no existe, usa '?'.
pub fn glyph(c: char) -> Option<RasterizedChar> {
    get_raster(c, WEIGHT, HEIGHT).or_else(|| get_raster('?', WEIGHT, HEIGHT))
}

/// Ancho de una celda de caracter (la fuente es monoespaciada).
pub fn char_width() -> i32 {
    get_raster('M', WEIGHT, HEIGHT).map(|g| g.width() as i32).unwrap_or(9)
}
