//! Decodificador de teclado PS/2 (scancode set 1): modificadores, teclas
//! extendidas, bloq mayus y tilde muerta.
use core::sync::atomic::{AtomicU8, Ordering::Relaxed};

use super::layout::Layout;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Delete,
    Home,
    End,
}

static LAYOUT: AtomicU8 = AtomicU8::new(Layout::Latam as u8);

pub fn set_layout(l: Layout) {
    LAYOUT.store(l as u8, Relaxed);
}

pub fn layout() -> Layout {
    Layout::from_u8(LAYOUT.load(Relaxed))
}

pub struct Keyboard {
    shift_l: bool,
    shift_r: bool,
    altgr: bool,
    ctrl: bool,
    caps: bool,
    e0: bool,
    dead_acute: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            shift_l: false,
            shift_r: false,
            altgr: false,
            ctrl: false,
            caps: false,
            e0: false,
            dead_acute: false,
        }
    }

    /// Recibe un byte del teclado; devuelve una tecla cuando esta completa.
    pub fn feed(&mut self, byte: u8) -> Option<Key> {
        if byte == 0xE0 {
            self.e0 = true;
            return None;
        }
        let ext = core::mem::take(&mut self.e0);
        let released = byte & 0x80 != 0;
        let code = byte & 0x7F;

        // modificadores
        match (ext, code) {
            (false, 0x2A) => {
                self.shift_l = !released;
                return None;
            }
            (false, 0x36) => {
                self.shift_r = !released;
                return None;
            }
            (true, 0x38) => {
                self.altgr = !released;
                return None;
            }
            (_, 0x1D) => {
                self.ctrl = !released;
                return None;
            }
            (false, 0x38) => return None, // Alt izquierdo
            (false, 0x3A) => {
                if !released {
                    self.caps = !self.caps;
                }
                return None;
            }
            _ => {}
        }
        if released {
            return None;
        }

        // teclas extendidas (flechas, etc.)
        if ext {
            return match code {
                0x48 => Some(Key::Up),
                0x50 => Some(Key::Down),
                0x4B => Some(Key::Left),
                0x4D => Some(Key::Right),
                0x53 => Some(Key::Delete),
                0x47 => Some(Key::Home),
                0x4F => Some(Key::End),
                0x1C => Some(Key::Enter),
                _ => None,
            };
        }

        // teclas de control
        let control = match code {
            0x01 => Some(Key::Escape),
            0x0E => Some(Key::Backspace),
            0x0F => Some(Key::Tab),
            0x1C => Some(Key::Enter),
            _ => None,
        };
        if control.is_some() {
            self.dead_acute = false;
            return control;
        }

        if self.ctrl {
            return None;
        }

        let shift = self.shift_l || self.shift_r;
        let layout = layout();

        // tilde muerta (solo en latam): la tecla a la derecha de la P
        if layout == Layout::Latam && code == 0x1A && !shift && !self.altgr {
            self.dead_acute = true;
            return None;
        }

        let mut ch = if code == 0x39 {
            ' '
        } else {
            layout.translate(code, shift, self.altgr)?
        };

        if self.caps && ch.is_alphabetic() {
            ch = swap_case(ch);
        }

        if self.dead_acute {
            self.dead_acute = false;
            if let Some(accented) = acute(ch) {
                return Some(Key::Char(accented));
            }
            if ch == ' ' {
                return Some(Key::Char('´'));
            }
        }
        Some(Key::Char(ch))
    }
}

fn swap_case(c: char) -> char {
    if c.is_lowercase() {
        c.to_uppercase().next().unwrap_or(c)
    } else {
        c.to_lowercase().next().unwrap_or(c)
    }
}

fn acute(c: char) -> Option<char> {
    Some(match c {
        'a' => 'á',
        'e' => 'é',
        'i' => 'í',
        'o' => 'ó',
        'u' => 'ú',
        'A' => 'Á',
        'E' => 'É',
        'I' => 'Í',
        'O' => 'Ó',
        'U' => 'Ú',
        _ => return None,
    })
}
