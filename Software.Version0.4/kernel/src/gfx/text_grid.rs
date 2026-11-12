//! Cuadricula de texto (como una terminal). No sabe de ventanas: solo
//! guarda caracteres y sabe dibujarse en un lienzo.
use alloc::{vec, vec::Vec};
use core::fmt::{self, Write};

use super::{
    canvas::Canvas,
    color::Color,
    font,
    rect::Rect,
};

pub const LINE_HEIGHT: i32 = 18;

#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    fg: Color,
}

pub struct TextGrid {
    cols: usize,
    rows: usize,
    cells: Vec<Cell>,
    cx: usize,
    cy: usize,
    fg: Color,
    default_fg: Color,
}

impl TextGrid {
    pub fn new(cols: usize, rows: usize, fg: Color) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell { ch: ' ', fg }; cols * rows],
            cx: 0,
            cy: 0,
            fg,
            default_fg: fg,
        }
    }

    pub fn set_color(&mut self, c: Color) {
        self.fg = c;
    }

    pub fn reset_color(&mut self) {
        self.fg = self.default_fg;
    }

    pub fn clear(&mut self) {
        let blank = Cell { ch: ' ', fg: self.default_fg };
        self.cells.fill(blank);
        self.cx = 0;
        self.cy = 0;
    }

    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            return self.newline();
        }
        if self.cx >= self.cols {
            self.newline();
        }
        self.cells[self.cy * self.cols + self.cx] = Cell { ch: c, fg: self.fg };
        self.cx += 1;
    }

    pub fn backspace(&mut self) {
        if self.cx > 0 {
            self.cx -= 1;
            self.cells[self.cy * self.cols + self.cx] = Cell { ch: ' ', fg: self.default_fg };
        }
    }

    fn newline(&mut self) {
        self.cx = 0;
        self.cy += 1;
        if self.cy >= self.rows {
            // subir todo una fila
            self.cells.copy_within(self.cols.., 0);
            let start = (self.rows - 1) * self.cols;
            let blank = Cell { ch: ' ', fg: self.default_fg };
            self.cells[start..].fill(blank);
            self.cy = self.rows - 1;
        }
    }

    /// Dibuja la cuadricula con su esquina superior izquierda en (x, y).
    pub fn draw(&self, canvas: &mut Canvas, x: i32, y: i32, cursor: Option<Color>) {
        let cw = font::char_width();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = self.cells[row * self.cols + col];
                if cell.ch != ' ' {
                    canvas.draw_char(
                        cell.ch,
                        x + col as i32 * cw,
                        y + row as i32 * LINE_HEIGHT,
                        cell.fg,
                    );
                }
            }
        }
        if let Some(color) = cursor {
            canvas.fill_rect(
                Rect::new(
                    x + self.cx as i32 * cw,
                    y + self.cy as i32 * LINE_HEIGHT + LINE_HEIGHT - 4,
                    cw,
                    2,
                ),
                color,
            );
        }
    }
}

impl Write for TextGrid {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| self.put_char(c));
        Ok(())
    }
}
