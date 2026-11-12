//! Shell: editor de linea con historial y autocompletado.
mod commands;

use alloc::{string::String, vec::Vec};
use core::fmt::Write;

use crate::drivers::keyboard::Key;
use crate::gfx::text_grid::TextGrid;
use crate::info::SystemInfo;
use crate::ui::theme;

// Menos que las columnas de la terminal, para que una linea no se parta.
const MAX_LINE: usize = 70;
const MAX_HISTORY: usize = 32;

pub struct Shell {
    line: String,
    history: Vec<String>,
    history_pos: Option<usize>,
}

impl Shell {
    pub fn new() -> Self {
        Self { line: String::new(), history: Vec::new(), history_pos: None }
    }

    pub fn show_prompt(&self, g: &mut TextGrid) {
        g.set_color(theme::ACCENT);
        let _ = write!(g, "mi_os> ");
        g.reset_color();
    }

    /// Devuelve true si la tecla cambio algo en pantalla.
    pub fn handle_key(&mut self, g: &mut TextGrid, key: Key, info: &SystemInfo) -> bool {
        match key {
            Key::Enter => {
                g.put_char('\n');
                let line = core::mem::take(&mut self.line);
                self.remember(&line);
                self.history_pos = None;
                commands::execute(g, &line, info);
                self.show_prompt(g);
            }
            Key::Backspace => {
                if self.line.pop().is_some() {
                    g.backspace();
                }
            }
            Key::Up => self.recall(g, true),
            Key::Down => self.recall(g, false),
            Key::Tab => self.complete(g),
            Key::Char(c) if !c.is_control() && self.line.chars().count() < MAX_LINE => {
                self.line.push(c);
                g.put_char(c);
            }
            _ => return false,
        }
        true
    }

    fn remember(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() || self.history.last().map(|l| l.as_str()) == Some(line) {
            return;
        }
        if self.history.len() >= MAX_HISTORY {
            self.history.remove(0);
        }
        self.history.push(String::from(line));
    }

    /// Reemplaza lo escrito por un comando del historial.
    fn recall(&mut self, g: &mut TextGrid, up: bool) {
        if self.history.is_empty() {
            return;
        }
        let last = self.history.len() - 1;
        let next = match (self.history_pos, up) {
            (None, true) => Some(last),
            (Some(0), true) => Some(0),
            (Some(i), true) => Some(i - 1),
            (Some(i), false) if i < last => Some(i + 1),
            (Some(_), false) => None,
            (None, false) => return,
        };

        while self.line.pop().is_some() {
            g.backspace();
        }
        self.history_pos = next;
        if let Some(i) = next {
            self.line = self.history[i].clone();
            for c in self.line.chars() {
                g.put_char(c);
            }
        }
    }

    /// Completa el nombre del comando si solo hay una opcion.
    fn complete(&mut self, g: &mut TextGrid) {
        if self.line.contains(' ') || self.line.is_empty() {
            return;
        }
        let mut matches = commands::COMMANDS.iter().filter(|c| c.starts_with(self.line.as_str()));
        let (Some(found), None) = (matches.next(), matches.next()) else { return };

        for c in found[self.line.len()..].chars() {
            self.line.push(c);
            g.put_char(c);
        }
    }
}
