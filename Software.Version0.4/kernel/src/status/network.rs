//! Red: busca adaptadores en el bus PCI.
use alloc::{format, string::String};

use super::{Indicator, Level};
use crate::drivers::pci::PciDevice;

const CLASS_NETWORK: u8 = 0x02;
const SUBCLASS_OTHER: u8 = 0x80; // aqui suelen caer las tarjetas Wi-Fi

pub fn probe(devices: &[PciDevice]) -> Indicator {
    let found = devices
        .iter()
        .find(|d| d.class == CLASS_NETWORK && d.subclass == SUBCLASS_OTHER)
        .or_else(|| devices.iter().find(|d| d.class == CLASS_NETWORK));

    match found {
        Some(d) => {
            let kind = if d.subclass == SUBCLASS_OTHER { "Wi-Fi" } else { "Ethernet" };
            Indicator {
                label: "Red",
                level: Level::Warn,
                detail: format!("{} {:04x}:{:04x} detectado, falta driver", kind, d.vendor, d.device),
            }
        }
        None => Indicator {
            label: "Red",
            level: Level::Off,
            detail: String::from("No se detecto ningun adaptador"),
        },
    }
}
