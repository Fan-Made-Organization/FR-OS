//! Fondo de pantalla. Se dibuja UNA vez y despues solo se copia.
use super::theme;
use crate::gfx::{canvas::Canvas, rect::Rect};

const BAND_H: i32 = 10;

pub fn render(width: usize, height: usize) -> Canvas {
    let mut c = Canvas::new(width, height);
    let (w, h) = (c.width(), c.height());

    // degradado + resplandores
    c.gradient_v(c.bounds(), theme::DESKTOP_TOP, theme::DESKTOP_BOTTOM);
    c.radial_glow(w / 2, h, h * 8 / 10, theme::GLOW_A, 200);
    c.radial_glow(w * 5 / 6, h / 5, h * 4 / 10, theme::GLOW_B, 90);

    // marca de agua grande y casi invisible
    let scale = 7;
    let text = "mi_os";
    let tw = c.text_width(text) * scale;
    c.draw_text_scaled(text, (w - tw) / 2, h / 2 - 8 * scale, scale, theme::HOT, 22);

    // lineas de barrido (aspecto de pantalla vieja)
    let mut y = 0;
    while y < h {
        c.shade_rect(Rect::new(0, y, w, 1), theme::HAZARD_DARK, 70);
        y += 3;
    }

    // franja de peligro abajo
    for y in (h - BAND_H)..h {
        for x in 0..w {
            let on = ((x + y) / 10) % 2 == 0;
            c.put(x, y, if on { theme::HAZARD } else { theme::HAZARD_DARK });
        }
    }
    c
}
