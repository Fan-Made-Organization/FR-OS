//! Ventana "Acerca de".
use super::App;
use crate::gfx::{canvas::Canvas, rect::Rect};
use crate::ui::theme;

const LINES: [(&str, bool); 7] = [
    ("MI_OS 4.0", true),
    ("", false),
    ("Sistema operativo escrito desde cero en Rust.", false),
    ("Kernel x86_64 con interrupciones, sin std.", false),
    ("", false),
    ("MI_OS (arriba a la izquierda): abre las apps.", false),
    ("Esquina derecha: centro de control.", false),
];

pub struct AboutApp;

impl App for AboutApp {
    fn title(&self) -> &str {
        "ACERCA DE"
    }

    fn content_size(&self) -> (i32, i32) {
        (560, 200)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, _focused: bool) {
        for (i, (text, accent)) in LINES.iter().enumerate() {
            let color = if *accent { theme::ACCENT } else { theme::TEXT };
            canvas.draw_text(text, area.x + 18, area.y + 18 + i as i32 * 24, color);
        }
    }
}
