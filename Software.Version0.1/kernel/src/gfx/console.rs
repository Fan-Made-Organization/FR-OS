//! Terminal de texto sobre el framebuffer.
use core::fmt::{self, Write};

use super::{font, framebuffer::Screen, Color, BG, FG};

const MARGIN: usize = 8;
const LINE_HEIGHT: usize = 18;

pub struct Terminal<'a> {
    screen: Screen<'a>,
    x: usize,
    y: usize,
    char_width: usize,
    color: Color,
}

impl<'a> Terminal<'a> {
    pub fn new(screen: Screen<'a>) -> Self {
        let mut t = Self {
            screen,
            x: MARGIN,
            y: MARGIN,
            char_width: font::char_width(),
            color: FG,
        };
        t.clear();
        t
    }

    pub fn clear(&mut self) {
        self.screen.clear(BG);
        self.x = MARGIN;
        self.y = MARGIN;
    }

    pub fn set_color(&mut self, c: Color) {
        self.color = c;
    }

    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            return self.newline();
        }
        if self.x + self.char_width > self.screen.width() {
            self.newline();
        }
        self.screen.draw_char(c, self.x, self.y, self.color, BG);
        self.x += self.char_width;
    }

    pub fn backspace(&mut self) {
        if self.x >= MARGIN + self.char_width {
            self.x -= self.char_width;
            self.screen
                .fill_rect(self.x, self.y, self.char_width, LINE_HEIGHT, BG);
        }
    }

    fn newline(&mut self) {
        self.x = MARGIN;
        self.y += LINE_HEIGHT;
        if self.y + LINE_HEIGHT > self.screen.height() {
            self.screen.scroll_up(LINE_HEIGHT);
            self.y -= LINE_HEIGHT;
        }
    }
}

impl Write for Terminal<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| self.put_char(c));
        Ok(())
    }
}
