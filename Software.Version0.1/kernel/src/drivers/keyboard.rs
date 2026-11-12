//! Teclado PS/2 (por polling; luego pasara a interrupciones).
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard as Decoder, ScancodeSet1};
use x86_64::instructions::port::Port;

const STATUS_PORT: u16 = 0x64;
const DATA_PORT: u16 = 0x60;

pub struct Keyboard {
    decoder: Decoder<layouts::Us104Key, ScancodeSet1>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            decoder: Decoder::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore),
        }
    }

    /// Devuelve una tecla ya decodificada si hay una disponible.
    pub fn poll(&mut self) -> Option<DecodedKey> {
        let scancode = read_scancode()?;
        let event = self.decoder.add_byte(scancode).ok()??;
        self.decoder.process_keyevent(event)
    }
}

fn read_scancode() -> Option<u8> {
    let mut status: Port<u8> = Port::new(STATUS_PORT);
    let mut data: Port<u8> = Port::new(DATA_PORT);
    unsafe {
        let s = status.read();
        if s & 0x01 == 0 {
            return None; // buffer vacio
        }
        let byte = data.read();
        if s & 0x20 != 0 {
            return None; // byte del raton: se ignora por ahora
        }
        Some(byte)
    }
}
