//! Bluetooth: casi siempre es un dispositivo USB, asi que primero hace
//! falta una pila USB. Mientras tanto solo podemos ver si hay controlador.
use alloc::string::String;

use super::{Indicator, Level};
use crate::drivers::pci::PciDevice;

const CLASS_WIRELESS: u8 = 0x0D;
const SUBCLASS_BLUETOOTH: u8 = 0x11;
const CLASS_SERIAL_BUS: u8 = 0x0C;
const SUBCLASS_USB: u8 = 0x03;

pub fn probe(devices: &[PciDevice]) -> Indicator {
    let (level, detail) = if devices
        .iter()
        .any(|d| d.class == CLASS_WIRELESS && d.subclass == SUBCLASS_BLUETOOTH)
    {
        (Level::Warn, "Bluetooth detectado, falta driver")
    } else if devices
        .iter()
        .any(|d| d.class == CLASS_SERIAL_BUS && d.subclass == SUBCLASS_USB)
    {
        (Level::Warn, "Controlador USB detectado, falta pila USB")
    } else {
        (Level::Off, "No hay controlador USB")
    };

    Indicator { label: "Bluetooth", level, detail: String::from(detail) }
}
