//! Distribuciones de teclado: de scancode (posicion fisica de la tecla)
//! a caracter. Hecho a mano para no depender de crates externos.

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Layout {
    Us = 0,
    Latam = 1,
}

impl Layout {
    pub fn from_u8(v: u8) -> Self {
        if v == 0 {
            Layout::Us
        } else {
            Layout::Latam
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Layout::Us => "us",
            Layout::Latam => "latam",
        }
    }

    pub fn translate(self, code: u8, shift: bool, altgr: bool) -> Option<char> {
        match self {
            Layout::Us => us(code, shift),
            Layout::Latam => latam(code, shift, altgr),
        }
    }
}

/// Letras: iguales en ambas distribuciones.
fn letter(code: u8) -> Option<char> {
    Some(match code {
        0x10 => 'q',
        0x11 => 'w',
        0x12 => 'e',
        0x13 => 'r',
        0x14 => 't',
        0x15 => 'y',
        0x16 => 'u',
        0x17 => 'i',
        0x18 => 'o',
        0x19 => 'p',
        0x1E => 'a',
        0x1F => 's',
        0x20 => 'd',
        0x21 => 'f',
        0x22 => 'g',
        0x23 => 'h',
        0x24 => 'j',
        0x25 => 'k',
        0x26 => 'l',
        0x2C => 'z',
        0x2D => 'x',
        0x2E => 'c',
        0x2F => 'v',
        0x30 => 'b',
        0x31 => 'n',
        0x32 => 'm',
        _ => return None,
    })
}

fn us(code: u8, shift: bool) -> Option<char> {
    if let Some(l) = letter(code) {
        return Some(if shift { l.to_ascii_uppercase() } else { l });
    }
    let (normal, shifted) = match code {
        0x02 => ('1', '!'),
        0x03 => ('2', '@'),
        0x04 => ('3', '#'),
        0x05 => ('4', '$'),
        0x06 => ('5', '%'),
        0x07 => ('6', '^'),
        0x08 => ('7', '&'),
        0x09 => ('8', '*'),
        0x0A => ('9', '('),
        0x0B => ('0', ')'),
        0x0C => ('-', '_'),
        0x0D => ('=', '+'),
        0x1A => ('[', '{'),
        0x1B => (']', '}'),
        0x27 => (';', ':'),
        0x28 => ('\'', '"'),
        0x29 => ('`', '~'),
        0x2B => ('\\', '|'),
        0x33 => (',', '<'),
        0x34 => ('.', '>'),
        0x35 => ('/', '?'),
        _ => return None,
    };
    Some(if shift { shifted } else { normal })
}

/// Latinoamericano (Honduras y la mayoria de LatAm).
fn latam(code: u8, shift: bool, altgr: bool) -> Option<char> {
    if altgr {
        let alt = match code {
            0x10 => Some('@'),
            0x02 => Some('|'),
            0x03 => Some('@'),
            0x04 => Some('#'),
            0x05 => Some('~'),
            0x08 => Some('{'),
            0x0B => Some('}'),
            0x0C => Some('\\'),
            0x1A => Some('['),
            0x1B => Some(']'),
            0x28 => Some('^'),
            0x2B => Some('`'),
            0x56 => Some('|'),
            _ => None,
        };
        if alt.is_some() {
            return alt;
        }
    }
    if let Some(l) = letter(code) {
        return Some(if shift { l.to_ascii_uppercase() } else { l });
    }
    let (normal, shifted) = match code {
        0x02 => ('1', '!'),
        0x03 => ('2', '"'),
        0x04 => ('3', '#'),
        0x05 => ('4', '$'),
        0x06 => ('5', '%'),
        0x07 => ('6', '&'),
        0x08 => ('7', '/'),
        0x09 => ('8', '('),
        0x0A => ('9', ')'),
        0x0B => ('0', '='),
        0x0C => ('\'', '?'),
        0x0D => ('¿', '¡'),
        0x1A => ('´', '¨'),
        0x1B => ('+', '*'),
        0x27 => ('ñ', 'Ñ'),
        0x28 => ('{', '['),
        0x29 => ('|', '°'),
        0x2B => ('}', ']'),
        0x33 => (',', ';'),
        0x34 => ('.', ':'),
        0x35 => ('-', '_'),
        0x56 => ('<', '>'),
        _ => return None,
    };
    Some(if shift { shifted } else { normal })
}
