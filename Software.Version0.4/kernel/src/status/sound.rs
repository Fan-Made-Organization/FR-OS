//! Sonido: busca controladores de audio en el bus PCI.
use alloc::{format, string::String};

use super::{Indicator, Level};
use crate::drivers::pci::PciDevice;

const CLASS_MULTIMEDIA: u8 = 0x04;

pub fn probe(devices: &[PciDevice]) -> Indicator {
    match devices.iter().find(|d| d.class == CLASS_MULTIMEDIA) {
        Some(d) => Indicator {
            label: "Sonido",
            level: Level::Warn,
            detail: format!("Audio {:04x}:{:04x} detectado, falta driver", d.vendor, d.device),
        },
        None => Indicator {
            label: "Sonido",
            level: Level::Off,
            detail: String::from("No se detecto controlador de audio"),
        },
    }
}
