//! Animaciones basadas en los ticks del temporizador (100 por segundo).

/// Progreso completo (la animacion termino).
pub const FULL: u32 = 256;

/// Duracion estandar de las animaciones: 150 ms.
pub const DURATION: u64 = 15;

#[derive(Clone, Copy)]
pub struct Tween {
    start: u64,
    duration: u64,
}

impl Tween {
    pub fn new(start: u64, duration: u64) -> Self {
        Self { start, duration }
    }

    /// 0..=FULL con suavizado (arranca rapido, termina lento).
    pub fn progress(&self, now: u64) -> u32 {
        let elapsed = now.saturating_sub(self.start);
        if elapsed >= self.duration {
            return FULL;
        }
        let t = (elapsed * FULL as u64 / self.duration) as u32;
        let u = FULL - t;
        FULL - u * u * u / (FULL * FULL)
    }

    pub fn done(&self, now: u64) -> bool {
        now.saturating_sub(self.start) >= self.duration
    }
}
