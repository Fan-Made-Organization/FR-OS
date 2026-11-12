//! Decodificador de paquetes del raton PS/2 (3 bytes por movimiento).

#[derive(Clone, Copy)]
pub struct MouseEvent {
    pub dx: i32,
    pub dy: i32, // positivo = hacia abajo
    pub left: bool,
    pub right: bool,
}

pub struct Mouse {
    packet: [u8; 3],
    index: usize,
}

/// Aceleracion suave: movimientos rapidos recorren mas pantalla.
fn accelerate(d: i32) -> i32 {
    let a = d.abs();
    let factor = if a > 14 {
        3
    } else if a > 6 {
        2
    } else {
        1
    };
    d * factor
}

impl Mouse {
    pub fn new() -> Self {
        Self { packet: [0; 3], index: 0 }
    }

    pub fn feed(&mut self, byte: u8) -> Option<MouseEvent> {
        // El primer byte siempre tiene el bit 3 en 1; si no, estamos desincronizados.
        if self.index == 0 && byte & 0x08 == 0 {
            return None;
        }
        self.packet[self.index] = byte;
        self.index += 1;
        if self.index < 3 {
            return None;
        }
        self.index = 0;

        let [flags, x, y] = self.packet;
        if flags & 0xC0 != 0 {
            return None; // desbordamiento
        }
        let dx = x as i32 - (((flags as i32) << 4) & 0x100);
        let dy = y as i32 - (((flags as i32) << 3) & 0x100);

        Some(MouseEvent {
            dx: accelerate(dx),
            dy: accelerate(-dy), // el raton manda "arriba" como positivo
            left: flags & 0x01 != 0,
            right: flags & 0x02 != 0,
        })
    }
}
