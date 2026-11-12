//! Terminal: una cuadricula de texto + el shell.
use core::fmt::Write;

use super::App;
use crate::drivers::keyboard::Key;
use crate::gfx::{
    canvas::Canvas,
    font,
    rect::Rect,
    text_grid::{TextGrid, LINE_HEIGHT},
};
use crate::info::SystemInfo;
use crate::shell::Shell;
use crate::ui::theme;

const COLS: usize = 80;
const ROWS: usize = 22;
const PAD: i32 = 8;

pub struct TerminalApp {
    grid: TextGrid,
    shell: Shell,
    info: SystemInfo,
}

impl TerminalApp {
    pub fn new(info: &SystemInfo) -> Self {
        let mut grid = TextGrid::new(COLS, ROWS, theme::TERM_FG);
        grid.set_color(theme::ACCENT);
        let _ = writeln!(grid, "=== MI_OS 4.0 ===");
        grid.reset_color();
        let _ = writeln!(grid, "Escribe 'help' para ver los comandos. Tab completa, flechas = historial.\n");

        let shell = Shell::new();
        shell.show_prompt(&mut grid);
        Self { grid, shell, info: *info }
    }
}

impl App for TerminalApp {
    fn title(&self) -> &str {
        "TERMINAL"
    }

    fn content_size(&self) -> (i32, i32) {
        (
            COLS as i32 * font::char_width() + 2 * PAD,
            ROWS as i32 * LINE_HEIGHT + 2 * PAD,
        )
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, focused: bool) {
        canvas.fill_rect(area, theme::TERM_BG);
        let cursor = if focused { Some(theme::ACCENT) } else { None };
        self.grid.draw(canvas, area.x + PAD, area.y + PAD, cursor);
    }

    fn on_key(&mut self, key: Key) -> bool {
        self.shell.handle_key(&mut self.grid, key, &self.info)
    }
}
