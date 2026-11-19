//! Cronometro: Espacio inicia o pausa, V marca una vuelta, R reinicia.
//! Usa el hook `tick` de las apps, que se llama muchas veces por segundo.
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "cronometro",
    version: "1.0.0",
    author: "FR-OS",
    description: "Cronometro con vueltas",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.app("Cronometro", Some(icono), nueva);
}

fn icono(canvas: &mut Canvas, tile: Rect, _bg: Color, fg: Color) {
    let cx = tile.x + tile.w / 2;
    let cy = tile.y + tile.h / 2;
    for r in [20, 19, 18] {
        canvas.circle(cx, cy + 3, r, fg, 255);
    }
    canvas.fill_rect(Rect::new(cx - 5, cy - 24, 10, 4), fg);
    for dx in 0..2 {
        canvas.line(cx + dx, cy + 3, cx + dx, cy - 10, fg, 255);
        canvas.line(cx + dx, cy + 3, cx + 9 + dx, cy + 8, fg, 255);
    }
}

fn nueva() -> Box<dyn App> {
    Box::new(Cronometro {
        corriendo: false,
        inicio: 0,
        acumulado: 0,
        mostrado: 0,
        vueltas: Vec::new(),
    })
}

struct Cronometro {
    corriendo: bool,
    inicio: u64,    // tick en que arranco la ultima vez
    acumulado: u64, // ticks de los tramos anteriores
    mostrado: u64,  // ultimo valor dibujado (en 1/20 de segundo)
    vueltas: Vec<String>,
}

impl Cronometro {
    fn transcurrido(&self, ahora: u64) -> u64 {
        self.acumulado + if self.corriendo { ahora - self.inicio } else { 0 }
    }

    fn centesimas(&self, ahora: u64) -> u64 {
        self.transcurrido(ahora) * 100 / TICKS_PER_SECOND
    }
}

fn texto(cs: u64) -> String {
    format!("{:02}:{:02}.{:02}", cs / 6000, (cs / 100) % 60, cs % 100)
}

impl App for Cronometro {
    fn title(&self) -> &str {
        "CRONOMETRO"
    }

    fn content_size(&self) -> (i32, i32) {
        (400, 300)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, _focused: bool) {
        canvas.fill_rect(area, theme::TERM_BG);

        let t = texto(self.centesimas(ticks()));
        let escala = 4;
        let ancho = canvas.text_width(&t) * escala;
        canvas.draw_text_scaled(&t, area.x + (area.w - ancho) / 2, area.y + 28, escala, theme::accent(), 255);

        canvas.draw_text(
            "Espacio: iniciar/pausar   V: vuelta   R: reiniciar",
            area.x + 16,
            area.y + 118,
            theme::MUTED,
        );
        for (i, vuelta) in self.vueltas.iter().rev().take(6).enumerate() {
            canvas.draw_text(vuelta, area.x + 16, area.y + 152 + i as i32 * 22, theme::TEXT);
        }
    }

    fn on_key(&mut self, key: Key) -> bool {
        let ahora = ticks();
        match key {
            Key::Char(' ') => {
                if self.corriendo {
                    self.acumulado = self.transcurrido(ahora);
                    self.corriendo = false;
                } else {
                    self.inicio = ahora;
                    self.corriendo = true;
                }
            }
            Key::Char('r') | Key::Char('R') => {
                self.corriendo = false;
                self.acumulado = 0;
                self.mostrado = 0;
                self.vueltas.clear();
            }
            Key::Char('v') | Key::Char('V') => {
                if self.corriendo {
                    let n = self.vueltas.len() + 1;
                    let t = texto(self.centesimas(ahora));
                    self.vueltas.push(format!("Vuelta {}: {}", n, t));
                }
            }
            _ => return false,
        }
        true
    }

    /// Redibuja unas 20 veces por segundo mientras corre.
    fn tick(&mut self, ahora: u64) -> bool {
        if !self.corriendo {
            return false;
        }
        let paso = self.centesimas(ahora) / 5;
        if paso != self.mostrado {
            self.mostrado = paso;
            true
        } else {
            false
        }
    }
}
