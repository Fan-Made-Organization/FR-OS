//! Lienzo en memoria: aqui se dibuja todo y luego se copia a la pantalla.
//! Tiene un rectangulo de recorte: nada se dibuja fuera de el. Asi se
//! puede redibujar solo la zona que cambio.
use alloc::{vec, vec::Vec};

use super::{
    color::{blend, Color},
    font,
    rect::Rect,
};

pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<u32>,
    clip: Rect,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let (w, h) = (width as i32, height as i32);
        Self {
            width: w,
            height: h,
            pixels: vec![0; width * height],
            clip: Rect::new(0, 0, w, h),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    // ───────── recorte ─────────

    pub fn clip(&self) -> Rect {
        self.clip
    }

    pub fn set_clip(&mut self, r: Rect) {
        self.clip = r.intersect(&self.bounds()).unwrap_or(Rect::new(0, 0, 0, 0));
    }

    pub fn reset_clip(&mut self) {
        self.clip = self.bounds();
    }

    // ───────── copia ─────────

    /// Copia una region de otro lienzo del mismo tamano (para el fondo).
    pub fn copy_region_from(&mut self, other: &Canvas, r: Rect) {
        if self.pixels.len() != other.pixels.len() {
            return;
        }
        let Some(r) = r.intersect(&self.clip) else { return };
        for y in r.y..r.bottom() {
            let row = (y * self.width) as usize;
            let a = row + r.x as usize;
            let b = row + r.right() as usize;
            self.pixels[a..b].copy_from_slice(&other.pixels[a..b]);
        }
    }

    // ───────── primitivas ─────────

    pub fn put(&mut self, x: i32, y: i32, c: Color) {
        if self.clip.contains(x, y) {
            self.pixels[(y * self.width + x) as usize] = c;
        }
    }

    pub fn fill_rect(&mut self, r: Rect, c: Color) {
        let Some(r) = r.intersect(&self.clip) else { return };
        for y in r.y..r.bottom() {
            let row = (y * self.width) as usize;
            self.pixels[row + r.x as usize..row + r.right() as usize].fill(c);
        }
    }

    /// Rectangulo semitransparente.
    pub fn shade_rect(&mut self, r: Rect, c: Color, alpha: u8) {
        let Some(r) = r.intersect(&self.clip) else { return };
        for y in r.y..r.bottom() {
            let row = (y * self.width) as usize;
            for px in &mut self.pixels[row + r.x as usize..row + r.right() as usize] {
                *px = blend(*px, c, alpha);
            }
        }
    }

    /// Degradado vertical.
    pub fn gradient_v(&mut self, r: Rect, top: Color, bottom: Color) {
        let denom = (r.h - 1).max(1);
        for i in 0..r.h {
            let t = (i * 255 / denom) as u8;
            self.fill_rect(Rect::new(r.x, r.y + i, r.w, 1), blend(top, bottom, t));
        }
    }

    /// Rectangulo con esquinas redondeadas. alpha 255 = solido.
    pub fn round_rect(&mut self, r: Rect, radius: i32, c: Color, alpha: u8) {
        let radius = radius.min(r.w / 2).min(r.h / 2).max(0);
        for dy in 0..r.h {
            let inset = corner_inset(dy, r.h, radius);
            let row = Rect::new(r.x + inset, r.y + dy, r.w - 2 * inset, 1);
            if alpha == 255 {
                self.fill_rect(row, c);
            } else {
                self.shade_rect(row, c, alpha);
            }
        }
    }

    /// Resplandor circular que se desvanece hacia afuera.
    pub fn radial_glow(&mut self, cx: i32, cy: i32, radius: i32, color: Color, strength: u8) {
        let area = Rect::new(cx - radius, cy - radius, radius * 2, radius * 2);
        let Some(a) = area.intersect(&self.clip) else { return };
        let r2 = (radius as i64) * (radius as i64);
        for y in a.y..a.bottom() {
            let dy = (y - cy) as i64;
            for x in a.x..a.right() {
                let dx = (x - cx) as i64;
                let d2 = dx * dx + dy * dy;
                if d2 >= r2 {
                    continue;
                }
                let t = ((r2 - d2) * 255 / r2) as u32;
                let alpha = (t * t / 255 * strength as u32 / 255) as u8;
                let i = (y * self.width + x) as usize;
                self.pixels[i] = blend(self.pixels[i], color, alpha);
            }
        }
    }

    /// Dibuja un sprite hecho de texto: '#' = pixel encendido.
    pub fn draw_sprite(&mut self, rows: &[&str], x: i32, y: i32, color: Color) {
        for (dy, row) in rows.iter().enumerate() {
            for (dx, b) in row.bytes().enumerate() {
                if b == b'#' {
                    self.put(x + dx as i32, y + dy as i32, color);
                }
            }
        }
    }

    // ───────── texto ─────────

    pub fn draw_char(&mut self, ch: char, x: i32, y: i32, fg: Color) {
        let Some(glyph) = font::glyph(ch) else { return };
        let bounds = Rect::new(x, y, glyph.width() as i32, glyph.height() as i32);
        let Some(vis) = bounds.intersect(&self.clip) else { return };

        let raster = glyph.raster();
        for py in vis.y..vis.bottom() {
            let row = raster[(py - y) as usize];
            for px in vis.x..vis.right() {
                let v = row[(px - x) as usize];
                if v == 0 {
                    continue;
                }
                let i = (py * self.width + px) as usize;
                self.pixels[i] = blend(self.pixels[i], fg, v);
            }
        }
    }

    /// Dibuja texto y devuelve su ancho en pixeles.
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, fg: Color) -> i32 {
        let cw = font::char_width();
        let mut cx = x;
        for ch in text.chars() {
            self.draw_char(ch, cx, y, fg);
            cx += cw;
        }
        cx - x
    }

    pub fn text_width(&self, text: &str) -> i32 {
        text.chars().count() as i32 * font::char_width()
    }

    pub fn draw_text_centered(&mut self, text: &str, r: Rect, fg: Color) {
        let tw = self.text_width(text);
        self.draw_text(
            text,
            r.x + (r.w - tw) / 2,
            r.y + (r.h - font::CHAR_HEIGHT) / 2,
            fg,
        );
    }

    /// Texto grande y translucido (marca de agua del fondo).
    pub fn draw_text_scaled(&mut self, text: &str, x: i32, y: i32, scale: i32, color: Color, alpha: u8) {
        let cw = font::char_width();
        let mut cx = x;
        for ch in text.chars() {
            if let Some(g) = font::glyph(ch) {
                for (dy, row) in g.raster().iter().enumerate() {
                    for (dx, &v) in row.iter().enumerate() {
                        if v < 64 {
                            continue;
                        }
                        let a = (v as u32 * alpha as u32 / 255) as u8;
                        let r = Rect::new(cx + dx as i32 * scale, y + dy as i32 * scale, scale, scale);
                        self.shade_rect(r, color, a);
                    }
                }
            }
            cx += cw * scale;
        }
    }
}

fn isqrt(n: i32) -> i32 {
    let mut x = 0;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// Cuantos pixeles se come la esquina redondeada en la fila `dy`.
fn corner_inset(dy: i32, h: i32, radius: i32) -> i32 {
    let dy = dy.min(h - 1 - dy);
    if dy >= radius {
        return 0;
    }
    let t = 2 * (radius - dy) - 1;
    let ext = isqrt(4 * radius * radius - t * t);
    radius - (ext + 1) / 2
}
