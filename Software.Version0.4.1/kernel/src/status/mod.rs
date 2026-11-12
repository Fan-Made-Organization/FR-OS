//! Estado del sistema que se muestra en la barra superior.
//! Cada indicador dice la verdad: si no hay driver, lo dice.
mod battery;
mod bluetooth;
mod memory;
mod network;
mod sound;

use alloc::{string::String, vec::Vec};

use crate::drivers::pci;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Funciona y se puede leer.
    Ok,
    /// El hardware existe pero falta el driver.
    Warn,
    /// No hay hardware o no se puede detectar.
    Off,
}

pub struct Indicator {
    pub label: &'static str,
    pub level: Level,
    pub detail: String,
}

pub struct Status {
    pub network: Indicator,
    pub bluetooth: Indicator,
    pub battery: Indicator,
    pub sound: Indicator,
    pub memory: Indicator,
}

impl Status {
    pub fn probe() -> Self {
        let mut devices = Vec::new();
        pci::scan(|d| devices.push(d));
        Self {
            network: network::probe(&devices),
            bluetooth: bluetooth::probe(&devices),
            battery: battery::probe(),
            sound: sound::probe(&devices),
            memory: memory::probe(),
        }
    }

    /// Vuelve a leer lo que cambia con el tiempo.
    pub fn refresh(&mut self) {
        self.memory = memory::probe();
    }

    pub fn all(&self) -> [&Indicator; 5] {
        [&self.network, &self.bluetooth, &self.battery, &self.sound, &self.memory]
    }
}
