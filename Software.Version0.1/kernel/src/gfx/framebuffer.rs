//! Dibujo de bajo nivel sobre el framebuffer.
use bootloader_api::info::{FrameBuffer, PixelFormat};

use super::{font, Color, BG};

pub struct Screen<'a> {
    buf: &'a mut [u8],
    width: usize,
    height: usize,
    stride: usize,
    bytes_per_pixel: usize,
    format: PixelFormat,
}

impl<'a> Screen<'a> {
    pub fn new(fb: &'a mut FrameBuffer) -> Self {
        let i = fb.info();
        Self {
            width: i.width,
            height: i.height,
            stride: i.stride,
            bytes_per_pixel: i.bytes_per_pixel,
            format: i.pixel_format,
            buf: fb.buffer_mut(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, (r, g, b): Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let o = (y * self.stride + x) * self.bytes_per_pixel;
        match self.format {
            PixelFormat::Rgb => {
                self.buf[o] = r;
                self.buf[o + 1] = g;
                self.buf[o + 2] = b;
            }
            PixelFormat::Bgr => {
                self.buf[o] = b;
                self.buf[o + 1] = g;
                self.buf[o + 2] = r;
            }
            _ => self.buf[o] = r,
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: Color) {
        for yy in y..(y + h).min(self.height) {
            for xx in x..(x + w).min(self.width) {
                self.put_pixel(xx, yy, c);
            }
        }
    }

    pub fn clear(&mut self, c: Color) {
        self.fill_rect(0, 0, self.width, self.height, c);
    }

    /// Sube todo el contenido `px` pixeles y limpia la franja de abajo.
    pub fn scroll_up(&mut self, px: usize) {
        let row = self.stride * self.bytes_per_pixel;
        let total = row * self.height;
        self.buf.copy_within(row * px..total, 0);
        self.fill_rect(0, self.height - px, self.width, px, BG);
    }

    pub fn draw_char(&mut self, ch: char, x: usize, y: usize, fg: Color, bg: Color) {
        let Some(glyph) = font::glyph(ch) else { return };
        for (dy, row) in glyph.raster().iter().enumerate() {
            for (dx, &v) in row.iter().enumerate() {
                self.put_pixel(x + dx, y + dy, blend(bg, fg, v));
            }
        }
    }
}

/// Mezcla fondo y texto segun la intensidad del glifo (anti-aliasing).
fn blend(bg: Color, fg: Color, v: u8) -> Color {
    let mix = |b: u8, f: u8| ((b as u16 * (255 - v as u16) + f as u16 * v as u16) / 255) as u8;
    (mix(bg.0, fg.0), mix(bg.1, fg.1), mix(bg.2, fg.2))
}
