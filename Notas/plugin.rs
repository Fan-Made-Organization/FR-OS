use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "notas",
    version: "1.0.0",
    author: "FR-OS",
    description: "Bloc de notas (no se guarda al reiniciar)",
    api_version: API_VERSION,
};

const FILAS: usize = 15; // lineas visibles
const ANCHO: usize = 58; // caracteres por linea

pub fn init(api: &mut Api) {
    api.app("Notas", Some(icono), nueva);
}

fn icono(canvas: &mut Canvas, tile: Rect, bg: Color, fg: Color) {
    let cx = tile.x + tile.w / 2;
    let cy = tile.y + tile.h / 2;
    let hoja = Rect::new(cx - 18, cy - 24, 36, 48);
    canvas.round_rect(hoja, 4, fg, 255);
    canvas.round_rect(hoja.inflate(-3), 2, bg, 255);
    for i in 0..5 {
        canvas.fill_rect(Rect::new(cx - 12, cy - 15 + i * 9, 24, 2), fg);
    }
}

fn nueva() -> Box<dyn App> {
    Box::new(Notas { lineas: alloc::vec![Vec::new()], fila: 0, col: 0, arriba: 0 })
}

struct Notas {
    lineas: Vec<Vec<char>>,
    fila: usize,
    col: usize,
    arriba: usize, // primera linea visible
}

impl Notas {
    /// Mantiene el cursor dentro de la zona visible.
    fn ajustar(&mut self) {
        if self.fila < self.arriba {
            self.arriba = self.fila;
        }
        if self.fila >= self.arriba + FILAS {
            self.arriba = self.fila + 1 - FILAS;
        }
    }
}

impl App for Notas {
    fn title(&self) -> &str {
        "NOTAS"
    }

    fn content_size(&self) -> (i32, i32) {
        (560, 350)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, focused: bool) {
        canvas.fill_rect(area, theme::TERM_BG);
        let ancho_letra = canvas.text_width("M");

        for (i, linea) in self.lineas.iter().enumerate().skip(self.arriba).take(FILAS) {
            let texto: String = linea.iter().collect();
            let y = area.y + 12 + (i - self.arriba) as i32 * 20;
            canvas.draw_text(&texto, area.x + 12, y, theme::TEXT);
        }

        if focused {
            let y = area.y + 12 + (self.fila - self.arriba) as i32 * 20;
            let x = area.x + 12 + self.col as i32 * ancho_letra;
            canvas.fill_rect(Rect::new(x, y, 2, 18), theme::accent());
        }

        canvas.draw_text(
            "Las notas se pierden al reiniciar (todavia no hay disco)",
            area.x + 12,
            area.bottom() - 24,
            theme::MUTED,
        );
    }

    fn on_key(&mut self, key: Key) -> bool {
        match key {
            Key::Char(c) if !c.is_control() => {
                if self.lineas[self.fila].len() < ANCHO {
                    self.lineas[self.fila].insert(self.col, c);
                    self.col += 1;
                }
            }
            Key::Tab => {
                for _ in 0..4 {
                    if self.lineas[self.fila].len() < ANCHO {
                        self.lineas[self.fila].insert(self.col, ' ');
                        self.col += 1;
                    }
                }
            }
            Key::Enter => {
                let resto = self.lineas[self.fila].split_off(self.col);
                self.fila += 1;
                self.lineas.insert(self.fila, resto);
                self.col = 0;
            }
            Key::Backspace => {
                if self.col > 0 {
                    self.col -= 1;
                    self.lineas[self.fila].remove(self.col);
                } else if self.fila > 0 {
                    let actual = self.lineas.remove(self.fila);
                    self.fila -= 1;
                    self.col = self.lineas[self.fila].len();
                    let sitio = ANCHO.saturating_sub(self.col);
                    self.lineas[self.fila].extend(actual.into_iter().take(sitio));
                }
            }
            Key::Delete => {
                if self.col < self.lineas[self.fila].len() {
                    self.lineas[self.fila].remove(self.col);
                } else if self.fila + 1 < self.lineas.len() {
                    let siguiente = self.lineas.remove(self.fila + 1);
                    let sitio = ANCHO.saturating_sub(self.lineas[self.fila].len());
                    self.lineas[self.fila].extend(siguiente.into_iter().take(sitio));
                }
            }
            Key::Left => {
                if self.col > 0 {
                    self.col -= 1;
                } else if self.fila > 0 {
                    self.fila -= 1;
                    self.col = self.lineas[self.fila].len();
                }
            }
            Key::Right => {
                if self.col < self.lineas[self.fila].len() {
                    self.col += 1;
                } else if self.fila + 1 < self.lineas.len() {
                    self.fila += 1;
                    self.col = 0;
                }
            }
            Key::Up => {
                if self.fila > 0 {
                    self.fila -= 1;
                    self.col = self.col.min(self.lineas[self.fila].len());
                }
            }
            Key::Down => {
                if self.fila + 1 < self.lineas.len() {
                    self.fila += 1;
                    self.col = self.col.min(self.lineas[self.fila].len());
                }
            }
            Key::Home => self.col = 0,
            Key::End => self.col = self.lineas[self.fila].len(),
            _ => return false,
        }
        self.ajustar();
        true
    }
}
