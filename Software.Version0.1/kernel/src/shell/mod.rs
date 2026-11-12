//! Shell: editor de linea + despacho de comandos.
mod commands;

use core::fmt::Write;
use pc_keyboard::DecodedKey;

use crate::gfx::{console::Terminal, ACCENT, FG};

const MAX_LINE: usize = 128;

/// Datos del sistema que los comandos pueden consultar.
pub struct SystemInfo {
    pub usable_kib: u64,
}

pub struct Shell {
    buf: [u8; MAX_LINE],
    len: usize,
}

impl Shell {
    pub fn new() -> Self {
        Self { buf: [0; MAX_LINE], len: 0 }
    }

    pub fn show_prompt(&self, t: &mut Terminal) {
        t.set_color(ACCENT);
        let _ = write!(t, "mi_os> ");
        t.set_color(FG);
    }

    pub fn handle_key(&mut self, t: &mut Terminal, key: DecodedKey, info: &SystemInfo) {
        let DecodedKey::Unicode(c) = key else { return };
        match c {
            '\n' => {
                t.put_char('\n');
                let line = core::str::from_utf8(&self.buf[..self.len]).unwrap_or("");
                commands::execute(t, line, info);
                self.len = 0;
                self.show_prompt(t);
            }
            '\u{8}' => {
                if self.len > 0 {
                    self.len -= 1;
                    t.backspace();
                }
            }
            c if c.is_ascii() && !c.is_control() && self.len < MAX_LINE => {
                self.buf[self.len] = c as u8;
                self.len += 1;
                t.put_char(c);
            }
            _ => {}
        }
    }
}
