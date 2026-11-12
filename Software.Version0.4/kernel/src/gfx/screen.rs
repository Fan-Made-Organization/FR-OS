//! La pantalla real (framebuffer). Copia pixeles y dibuja el cursor.
use bootloader_api::info::{FrameBuffer, PixelFormat};

use super::{
    canvas::Canvas,
    color::{channels, Color},
    cursor,
    rect::Rect,
};
use crate::crash;

pub struct Screen<'a> {
    buf: &'a mut [u8],
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
    // posicion de cada canal dentro del pixel
    ri: usize,
    gi: usize,
    bi: usize,
    /// true si el formato coincide con el del lienzo y se puede copiar por filas.
    fast: bool,
}

impl<'a> Screen<'a> {
    pub fn new(fb: &'a mut FrameBuffer) -> Self {
        let i = fb.info();
        let (ri, gi, bi) = match i.pixel_format {
            PixelFormat::Rgb => (0, 1, 2),
            PixelFormat::U8 => (0, 0, 0),
            _ => (2, 1, 0), // Bgr y desconocidos
        };
        let (width, height, stride, bpp) = (i.width, i.height, i.stride, i.bytes_per_pixel);

        let buf = fb.buffer_mut();
        crash::register(buf.as_mut_ptr(), width, height, stride, bpp, (ri, gi, bi));

        Self {
            buf,
            width,
            height,
            stride,
            bpp,
            ri,
            gi,
            bi,
            // el lienzo guarda 0x00RRGGBB: en memoria little-endian eso es B,G,R,0
            fast: bpp == 4 && (ri, gi, bi) == (2, 1, 0),
        }
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }

    #[inline(always)]
    fn write(&mut self, offset: usize, c: Color) {
        let (r, g, b) = channels(c);
        self.buf[offset + self.ri] = r;
        self.buf[offset + self.gi] = g;
        self.buf[offset + self.bi] = b;
    }

    /// Copia una region del lienzo a la pantalla.
    pub fn present(&mut self, canvas: &Canvas, area: Rect) {
        let screen = Rect::new(0, 0, self.width as i32, self.height as i32);
        let Some(bounds) = screen.intersect(&canvas.bounds()) else { return };
        let Some(r) = area.intersect(&bounds) else { return };

        let cw = canvas.width() as usize;
        for y in r.y..r.bottom() {
            let row = y as usize * cw;
            let src = &canvas.pixels()[row + r.x as usize..row + r.right() as usize];
            let mut o = (y as usize * self.stride + r.x as usize) * self.bpp;

            if self.fast {
                // una sola copia de memoria por fila
                let bytes =
                    unsafe { core::slice::from_raw_parts(src.as_ptr() as *const u8, src.len() * 4) };
                self.buf[o..o + bytes.len()].copy_from_slice(bytes);
            } else {
                for &c in src {
                    self.write(o, c);
                    o += self.bpp;
                }
            }
        }
    }

    /// Dibuja el cursor directamente encima de la pantalla.
    pub fn draw_cursor(&mut self, x: i32, y: i32) {
        for row in 0..cursor::HEIGHT {
            for col in 0..cursor::WIDTH {
                let Some(c) = cursor::pixel(col, row) else { continue };
                let (px, py) = (x + col, y + row);
                if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
                    continue;
                }
                let o = (py as usize * self.stride + px as usize) * self.bpp;
                self.write(o, c);
            }
        }
    }
}
