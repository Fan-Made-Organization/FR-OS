//! Puerto serie COM1: el "log" del kernel. Con `-serial stdio` aparece en
//! la terminal donde corres `cargo run`, aunque la pantalla falle.
use core::fmt::{self, Write};
use x86_64::instructions::port::Port;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        Port::<u8>::new(COM1 + 1).write(0x00); // sin interrupciones
        Port::<u8>::new(COM1 + 3).write(0x80); // acceso al divisor
        Port::<u8>::new(COM1).write(0x03); // 38400 baudios
        Port::<u8>::new(COM1 + 1).write(0x00);
        Port::<u8>::new(COM1 + 3).write(0x03); // 8 bits, sin paridad
        Port::<u8>::new(COM1 + 2).write(0xC7); // FIFO
        Port::<u8>::new(COM1 + 4).write(0x0B);
    }
}

fn write_byte(b: u8) {
    unsafe {
        let mut status = Port::<u8>::new(COM1 + 5);
        for _ in 0..100_000 {
            if status.read() & 0x20 != 0 {
                break;
            }
        }
        Port::<u8>::new(COM1).write(b);
    }
}

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' {
                write_byte(b'\r');
            }
            write_byte(b);
        }
        Ok(())
    }
}

pub fn log(args: fmt::Arguments) {
    let _ = Serial.write_fmt(args);
}

/// `crate::log!("texto {}", valor)` escribe una linea en el puerto serie.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::drivers::serial::log(format_args!($($arg)*));
        $crate::drivers::serial::log(format_args!("\n"));
    }};
}
