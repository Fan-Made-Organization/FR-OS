//! Calculadora: una app en el lanzador y el comando `calc`.
//! Entiende + - * / parentesis y decimales, con la precedencia normal.
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "calculadora",
    version: "1.0.0",
    author: "FR-OS",
    description: "App de calculadora y comando calc",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.app("Calculadora", Some(icono), nueva);
    api.command("calc", "calcula una expresion: calc 2+3*4", comando);
}

fn icono(canvas: &mut Canvas, tile: Rect, bg: Color, fg: Color) {
    let cx = tile.x + tile.w / 2;
    let cy = tile.y + tile.h / 2;
    let cuerpo = Rect::new(cx - 20, cy - 26, 40, 52);
    canvas.round_rect(cuerpo, 5, fg, 255);
    canvas.round_rect(cuerpo.inflate(-3), 3, bg, 255);
    canvas.fill_rect(Rect::new(cx - 15, cy - 21, 30, 10), fg);
    for fila in 0..3 {
        for col in 0..3 {
            canvas.fill_rect(Rect::new(cx - 15 + col * 11, cy - 5 + fila * 10, 8, 6), fg);
        }
    }
}

fn nueva() -> Box<dyn App> {
    Box::new(Calculadora { entrada: String::new(), historial: Vec::new() })
}

fn comando(out: &mut dyn Write, args: &str) {
    match evaluar(args) {
        Ok(v) => {
            let _ = writeln!(out, "{} = {}", args, formato(v));
        }
        Err(e) => {
            let _ = writeln!(out, "error: {}", e);
        }
    }
}

// ───────── la ventana ─────────

struct Calculadora {
    entrada: String,
    historial: Vec<String>,
}

fn permitido(c: char) -> bool {
    c.is_ascii_digit() || "+-*/(). ".contains(c)
}

impl App for Calculadora {
    fn title(&self) -> &str {
        "CALCULADORA"
    }

    fn content_size(&self) -> (i32, i32) {
        (420, 300)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, focused: bool) {
        canvas.fill_rect(area, theme::TERM_BG);

        let desde = self.historial.len().saturating_sub(9);
        for (i, linea) in self.historial[desde..].iter().enumerate() {
            canvas.draw_text(linea, area.x + 16, area.y + 14 + i as i32 * 24, theme::TEXT);
        }

        let caja = Rect::new(area.x + 12, area.bottom() - 48, area.w - 24, 34);
        canvas.round_rect(caja.inflate(1), theme::RADIUS + 1, theme::OUTLINE, 255);
        canvas.round_rect(caja, theme::RADIUS, theme::TILE_BG, 255);
        let y = caja.y + (caja.h - 16) / 2;
        let ancho = canvas.draw_text(&self.entrada, caja.x + 10, y, theme::TEXT);
        if focused {
            canvas.fill_rect(Rect::new(caja.x + 10 + ancho + 2, y, 2, 16), theme::accent());
        }
    }

    fn on_key(&mut self, key: Key) -> bool {
        match key {
            Key::Char(c) if permitido(c) && self.entrada.chars().count() < 40 => self.entrada.push(c),
            Key::Backspace => {
                self.entrada.pop();
            }
            Key::Enter => {
                if !self.entrada.trim().is_empty() {
                    let linea = match evaluar(&self.entrada) {
                        Ok(v) => format!("{} = {}", self.entrada, formato(v)),
                        Err(e) => format!("{}  ->  error: {}", self.entrada, e),
                    };
                    self.historial.push(linea);
                    self.entrada.clear();
                }
            }
            _ => return false,
        }
        true
    }
}

// ───────── el evaluador ─────────

/// Numero sin ceros sobrantes: 14.000000 -> 14, 0.500000 -> 0.5
fn formato(v: f64) -> String {
    let mut s = format!("{:.6}", v);
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    if s == "-0" {
        s = String::from("0");
    }
    s
}

struct Lector<'a> {
    texto: &'a [u8],
    pos: usize,
}

impl Lector<'_> {
    fn mirar(&self) -> Option<u8> {
        self.texto.get(self.pos).copied()
    }

    fn saltar_espacios(&mut self) {
        while self.mirar() == Some(b' ') {
            self.pos += 1;
        }
    }

    // expresion = termino (('+' | '-') termino)*
    fn expresion(&mut self) -> Result<f64, &'static str> {
        let mut v = self.termino()?;
        loop {
            self.saltar_espacios();
            match self.mirar() {
                Some(b'+') => {
                    self.pos += 1;
                    v += self.termino()?;
                }
                Some(b'-') => {
                    self.pos += 1;
                    v -= self.termino()?;
                }
                _ => return Ok(v),
            }
        }
    }

    // termino = factor (('*' | '/') factor)*
    fn termino(&mut self) -> Result<f64, &'static str> {
        let mut v = self.factor()?;
        loop {
            self.saltar_espacios();
            match self.mirar() {
                Some(b'*') => {
                    self.pos += 1;
                    v *= self.factor()?;
                }
                Some(b'/') => {
                    self.pos += 1;
                    let d = self.factor()?;
                    if d == 0.0 {
                        return Err("division por cero");
                    }
                    v /= d;
                }
                _ => return Ok(v),
            }
        }
    }

    // factor = '-' factor | '(' expresion ')' | numero
    fn factor(&mut self) -> Result<f64, &'static str> {
        self.saltar_espacios();
        match self.mirar() {
            Some(b'-') => {
                self.pos += 1;
                Ok(-self.factor()?)
            }
            Some(b'(') => {
                self.pos += 1;
                let v = self.expresion()?;
                self.saltar_espacios();
                if self.mirar() == Some(b')') {
                    self.pos += 1;
                    Ok(v)
                } else {
                    Err("falta cerrar el parentesis")
                }
            }
            Some(c) if c.is_ascii_digit() || c == b'.' => self.numero(),
            _ => Err("expresion invalida"),
        }
    }

    fn numero(&mut self) -> Result<f64, &'static str> {
        let mut v = 0.0;
        let mut hay_digitos = false;
        while let Some(c) = self.mirar() {
            if c.is_ascii_digit() {
                v = v * 10.0 + (c - b'0') as f64;
                self.pos += 1;
                hay_digitos = true;
            } else {
                break;
            }
        }
        if self.mirar() == Some(b'.') {
            self.pos += 1;
            let mut escala = 0.1;
            while let Some(c) = self.mirar() {
                if c.is_ascii_digit() {
                    v += (c - b'0') as f64 * escala;
                    escala /= 10.0;
                    self.pos += 1;
                    hay_digitos = true;
                } else {
                    break;
                }
            }
        }
        if hay_digitos {
            Ok(v)
        } else {
            Err("numero invalido")
        }
    }
}

fn evaluar(texto: &str) -> Result<f64, &'static str> {
    let mut lector = Lector { texto: texto.as_bytes(), pos: 0 };
    let v = lector.expresion()?;
    lector.saltar_espacios();
    if lector.pos != lector.texto.len() {
        return Err("sobran caracteres");
    }
    Ok(v)
}
