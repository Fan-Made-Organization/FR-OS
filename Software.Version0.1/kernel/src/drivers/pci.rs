//! Escaneo del bus PCI.
use x86_64::instructions::port::Port;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
}

impl PciDevice {
    pub fn class_name(&self) -> &'static str {
        match self.class {
            0x01 => "almacenamiento",
            0x02 => "red",
            0x03 => "pantalla/GPU",
            0x04 => "multimedia/audio",
            0x06 => "puente",
            0x0C => "bus serie (USB...)",
            _ => "otro",
        }
    }
}

fn read32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let addr = 0x8000_0000u32
        | (bus as u32) << 16
        | (slot as u32) << 11
        | (func as u32) << 8
        | (offset as u32 & 0xFC);
    unsafe {
        Port::<u32>::new(CONFIG_ADDRESS).write(addr);
        Port::<u32>::new(CONFIG_DATA).read()
    }
}

/// Recorre todos los dispositivos PCI y llama a `visit` por cada uno.
pub fn scan<F: FnMut(PciDevice)>(mut visit: F) {
    for bus in 0..=255u8 {
        for slot in 0..32u8 {
            if read32(bus, slot, 0, 0) & 0xFFFF == 0xFFFF {
                continue;
            }
            let multifunction = (read32(bus, slot, 0, 0x0C) >> 16) & 0x80 != 0;
            let funcs = if multifunction { 8 } else { 1 };

            for func in 0..funcs {
                let id = read32(bus, slot, func, 0);
                if id & 0xFFFF == 0xFFFF {
                    continue;
                }
                visit(PciDevice {
                    bus,
                    slot,
                    func,
                    vendor: (id & 0xFFFF) as u16,
                    device: (id >> 16) as u16,
                    class: (read32(bus, slot, func, 0x08) >> 24) as u8,
                });
            }
        }
    }
}
