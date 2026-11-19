//! Viborita: el clasico. Flechas (o WASD) para girar, Enter para reiniciar.
//! Usa el hook `tick` de las apps para avanzar sola.
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "viborita",
    version: "1.0.0",
    author: "FR-OS",
    description: "Juego de la viborita",
    api_version: API_VERSION,
};

const COLS: i32 = 24;
const FILAS: i32 = 16;
const CELDA: i32 = 18;
const CABECERA: i32 = 30;

pub fn init(api: &mut Api) {
    api.app("Viborita", Some(icono), nueva);
}

fn icono(canvas: &mut Canvas, tile: Rect, _bg: Color, fg: Color) {
    let cx = tile.x + tile.w / 2;
    let cy = tile.y + tile.h / 2;
    for (dx, dy) in [(-18, -10), (-8, -10), (2, -10), (2, 0), (-8, 0), (-18, 10), (-8, 10)] {
        canvas.fill_rect(Rect::new(cx + dx, cy + dy, 9, 9), fg);
    }
    canvas.round_rect(Rect::new(cx + 14, cy - 12, 8, 8), 3, fg, 255);
}

fn nueva() -> Box<dyn App> {
    Box::new(Viborita::nueva())
}

struct Viborita {
    cuerpo: Vec<(i32, i32)>, // la cabeza es el primero
    dir: (i32, i32),
    pendiente: (i32, i32),
    comida: (i32, i32),
    puntos: u32,
    vivo: bool,
    ultimo: u64,
    semilla: u32,
}

impl Viborita {
    fn nueva() -> Self {
        let mut v = Viborita {
            cuerpo: alloc::vec![(5, 8), (4, 8), (3, 8)],
            dir: (1, 0),
            pendiente: (1, 0),
            comida: (0, 0),
            puntos: 0,
            vivo: true,
            ultimo: ticks(),
            semilla: (ticks() as u32) | 1,
        };
        v.colocar_comida();
        v
    }

    fn azar(&mut self) -> u32 {
        self.semilla ^= self.semilla << 13;
        self.semilla ^= self.semilla >> 17;
        self.semilla ^= self.semilla << 5;
        self.semilla
    }

    fn colocar_comida(&mut self) {
        if self.cuerpo.len() >= (COLS * FILAS) as usize {
            self.vivo = false; // ganaste: no queda sitio
            return;
        }
        loop {
            let x = (self.azar() % COLS as u32) as i32;
            let y = (self.azar() % FILAS as u32) as i32;
            if !self.cuerpo.contains(&(x, y)) {
                self.comida = (x, y);
                return;
            }
        }
    }

    fn paso(&mut self) {
        self.dir = self.pendiente;
        let (cx, cy) = self.cuerpo[0];
        let nueva = (cx + self.dir.0, cy + self.dir.1);

        let choca = nueva.0 < 0
            || nueva.1 < 0
            || nueva.0 >= COLS
            || nueva.1 >= FILAS
            || self.cuerpo.contains(&nueva);
        if choca {
            self.vivo = false;
            return;
        }

        self.cuerpo.insert(0, nueva);
        if nueva == self.comida {
            self.puntos += 1;
            self.colocar_comida();
        } else {
            self.cuerpo.pop();
        }
    }
}

impl App for Viborita {
    fn title(&self) -> &str {
        "VIBORITA"
    }

    fn content_size(&self) -> (i32, i32) {
        (COLS * CELDA, FILAS * CELDA + CABECERA)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, _focused: bool) {
        canvas.fill_rect(area, theme::TERM_BG);
        let marcador = format!("Puntos: {}", self.puntos);
        canvas.draw_text(&marcador, area.x + 12, area.y + 7, theme::accent());

        let tablero = Rect::new(area.x, area.y + CABECERA, COLS * CELDA, FILAS * CELDA);
        canvas.fill_rect(tablero, theme::WINDOW_BG);

        let celda = |x: i32, y: i32| {
            Rect::new(tablero.x + x * CELDA + 1, tablero.y + y * CELDA + 1, CELDA - 2, CELDA - 2)
        };
        canvas.round_rect(celda(self.comida.0, self.comida.1), 5, theme::TEXT, 255);
        for (i, &(x, y)) in self.cuerpo.iter().enumerate() {
            let color = if i == 0 { theme::TEXT } else { theme::accent() };
            canvas.round_rect(celda(x, y), 3, color, 255);
        }

        if !self.vivo {
            let aviso = "Fin del juego - Enter para reiniciar";
            let ancho = canvas.text_width(aviso);
            let caja = Rect::new(tablero.x + (tablero.w - ancho) / 2 - 12, tablero.y + 110, ancho + 24, 36);
            canvas.round_rect(caja, theme::RADIUS, theme::PANEL_BG, 255);
            canvas.draw_text_centered(aviso, caja, theme::TEXT);
        }
    }

    fn on_key(&mut self, key: Key) -> bool {
        match key {
            Key::Up | Key::Char('w') | Key::Char('W') => {
                if self.dir != (0, 1) {
                    self.pendiente = (0, -1);
                }
            }
            Key::Down | Key::Char('s') | Key::Char('S') => {
                if self.dir != (0, -1) {
                    self.pendiente = (0, 1);
                }
            }
            Key::Left | Key::Char('a') | Key::Char('A') => {
                if self.dir != (1, 0) {
                    self.pendiente = (-1, 0);
                }
            }
            Key::Right | Key::Char('d') | Key::Char('D') => {
                if self.dir != (-1, 0) {
                    self.pendiente = (1, 0);
                }
            }
            Key::Enter => {
                if !self.vivo {
                    *self = Viborita::nueva();
                }
            }
            _ => return false,
        }
        true
    }

    /// Avanza sola: mas rapido a medida que sumas puntos.
    fn tick(&mut self, ahora: u64) -> bool {
        if !self.vivo {
            return false;
        }
        let base = TICKS_PER_SECOND / 8;
        let intervalo = base
            .saturating_sub(self.puntos as u64 * TICKS_PER_SECOND / 100)
            .max(TICKS_PER_SECOND / 16);
        if ahora - self.ultimo >= intervalo {
            self.ultimo = ahora;
            self.paso();
            true
        } else {
            false
        }
    }
}
