//! Pantalla de error fatal. Dibuja directo sobre el framebuffer, sin heap
//! ni nada del escritorio, para que funcione aunque todo lo demas falle.
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering::Relaxed};

use crate::drivers::serial;
use crate::gfx::{
    color::{channels, rgb, Color},
    font,
};

static FB: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
static WIDTH: AtomicUsize = AtomicUsize::new(0);
static HEIGHT: AtomicUsize = AtomicUsize::new(0);
static STRIDE: AtomicUsize = AtomicUsize::new(0);
static BPP: AtomicUsize = AtomicUsize::new(0);
static RI: AtomicUsize = AtomicUsize::new(0);
static GI: AtomicUsize = AtomicUsize::new(1);
static BI: AtomicUsize = AtomicUsize::new(2);

const BG: Color = rgb(18, 0, 4);
const HOT: Color = rgb(255, 50, 70);
const TEXT: Color = rgb(240, 230, 232);
const MUTED: Color = rgb(170, 140, 146);
const LINE_H: usize = 18;

/// La pantalla le avisa a este modulo donde esta el framebuffer.
pub fn register(
    ptr: *mut u8,
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
    order: (usize, usize, usize),
) {
    WIDTH.store(width, Relaxed);
    HEIGHT.store(height, Relaxed);
    STRIDE.store(stride, Relaxed);
    BPP.store(bpp, Relaxed);
    RI.store(order.0, Relaxed);
    GI.store(order.1, Relaxed);
    BI.store(order.2, Relaxed);
    FB.store(ptr, Relaxed);
}

fn put(x: usize, y: usize, c: Color) {
    let p = FB.load(Relaxed);
    if p.is_null() || x >= WIDTH.load(Relaxed) || y >= HEIGHT.load(Relaxed) {
        return;
    }
    let (r, g, b) = channels(c);
    let o = (y * STRIDE.load(Relaxed) + x) * BPP.load(Relaxed);
    unsafe {
        p.add(o + RI.load(Relaxed)).write_volatile(r);
        p.add(o + GI.load(Relaxed)).write_volatile(g);
        p.add(o + BI.load(Relaxed)).write_volatile(b);
    }
}

fn fill(c: Color) {
    for y in 0..HEIGHT.load(Relaxed) {
        for x in 0..WIDTH.load(Relaxed) {
            put(x, y, c);
        }
    }
}

struct Painter {
    x: usize,
    y: usize,
    left: usize,
    color: Color,
}

impl Write for Painter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let cw = font::char_width() as usize;
        for ch in s.chars() {
            if ch == '\n' {
                self.x = self.left;
                self.y += LINE_H;
                continue;
            }
            if self.x + cw > WIDTH.load(Relaxed) {
                self.x = self.left;
                self.y += LINE_H;
            }
            if let Some(g) = font::glyph(ch) {
                for (dy, row) in g.raster().iter().enumerate() {
                    for (dx, &v) in row.iter().enumerate() {
                        if v > 90 {
                            put(self.x + dx, self.y + dy, self.color);
                        }
                    }
                }
            }
            self.x += cw;
        }
        Ok(())
    }
}

/// Muestra el error y detiene la maquina.
pub fn show(title: &str, details: fmt::Arguments) -> ! {
    x86_64::instructions::interrupts::disable();
    serial::log(format_args!("\n=== {} ===\n{}\n", title, details));

    if !FB.load(Relaxed).is_null() {
        fill(BG);
        let mut p = Painter { x: 32, y: 32, left: 32, color: HOT };
        let _ = write!(p, "!! LA AMENAZA GANO !!\n\n{}\n\n", title);
        p.color = TEXT;
        let _ = write!(p, "{}\n\n", details);
        p.color = MUTED;
        let _ = write!(p, "El kernel se detuvo. Reinicia la maquina (QEMU: Machine > Reset).");
    }

    loop {
        x86_64::instructions::hlt();
    }
}
