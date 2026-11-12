//! Una ventana: marco + barra de titulo + una aplicacion adentro.
use alloc::boxed::Box;

use super::theme;
use crate::apps::App;
use crate::gfx::{canvas::Canvas, font, rect::Rect};

pub const TITLE_H: i32 = 28;
pub const BORDER: i32 = 1;
const SHADOW: i32 = 6;

pub struct Window {
    pub id: u32,
    pub rect: Rect,
    pub app: Box<dyn App>,
}

impl Window {
    pub fn new(id: u32, app: Box<dyn App>, x: i32, y: i32) -> Self {
        let (cw, ch) = app.content_size();
        Self {
            id,
            rect: Rect::new(x, y, cw + 2 * BORDER, ch + TITLE_H + BORDER),
            app,
        }
    }

    pub fn title(&self) -> &str {
        self.app.title()
    }

    /// Area que ocupa incluyendo la sombra.
    pub fn outer_rect(&self) -> Rect {
        Rect::new(self.rect.x, self.rect.y, self.rect.w + SHADOW, self.rect.h + SHADOW)
    }

    pub fn title_rect(&self) -> Rect {
        Rect::new(
            self.rect.x + BORDER,
            self.rect.y + BORDER,
            self.rect.w - 2 * BORDER,
            TITLE_H - BORDER,
        )
    }

    pub fn close_rect(&self) -> Rect {
        let t = self.title_rect();
        Rect::new(t.right() - 28, t.y + 4, 22, t.h - 8)
    }

    pub fn content_rect(&self) -> Rect {
        Rect::new(
            self.rect.x + BORDER,
            self.rect.y + TITLE_H,
            self.rect.w - 2 * BORDER,
            self.rect.h - TITLE_H - BORDER,
        )
    }

    pub fn draw(&mut self, canvas: &mut Canvas, focused: bool) {
        let r = self.rect;

        // sombra (solo los dos bordes que asoman)
        canvas.shade_rect(Rect::new(r.right(), r.y + SHADOW, SHADOW, r.h), theme::SHADOW, 120);
        canvas.shade_rect(Rect::new(r.x + SHADOW, r.bottom(), r.w - SHADOW, SHADOW), theme::SHADOW, 120);

        // marco: rojo si la ventana esta activa
        let border = if focused { theme::HOT } else { theme::WINDOW_BORDER };
        canvas.fill_rect(r, border);

        let t = self.title_rect();
        if focused {
            canvas.gradient_v(t, theme::TITLE_FOCUSED_TOP, theme::TITLE_FOCUSED_BOTTOM);
        } else {
            canvas.fill_rect(t, theme::TITLE_IDLE);
        }
        canvas.draw_text(
            self.app.title(),
            t.x + 12,
            t.y + (t.h - font::CHAR_HEIGHT) / 2,
            theme::TITLE_TEXT,
        );

        let close = self.close_rect();
        canvas.round_rect(close, 4, theme::HOT_SOFT, 255);
        canvas.draw_text_centered("x", close, theme::ON_HOT);

        let content = self.content_rect();
        canvas.fill_rect(content, theme::WINDOW_BG);
        self.app.draw(canvas, content, focused);
    }
}
