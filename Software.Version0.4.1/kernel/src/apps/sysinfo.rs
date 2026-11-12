//! Ventana "Sistema": resumen del hardware.
use alloc::{format, string::String, vec::Vec};

use super::App;
use crate::arch::cpu::CpuInfo;
use crate::drivers::{keyboard, pci};
use crate::gfx::{canvas::Canvas, rect::Rect};
use crate::info::SystemInfo;
use crate::ui::theme;

const VALUE_X: i32 = 170;

pub struct SysInfoApp {
    rows: Vec<(&'static str, String)>,
}

impl SysInfoApp {
    pub fn new(info: &SystemInfo) -> Self {
        let cpu = CpuInfo::read();
        let mut pci_count = 0;
        pci::scan(|_| pci_count += 1);

        let rows = alloc::vec![
            ("Procesador", String::from(cpu.brand())),
            ("Fabricante", String::from(cpu.vendor())),
            ("Memoria", format!("{} MiB utilizables", info.usable_kib / 1024)),
            ("Pantalla", format!("{} x {}", info.screen_width, info.screen_height)),
            ("Dispositivos PCI", format!("{}", pci_count)),
            ("Teclado", format!("distribucion {}", keyboard::layout().name())),
        ];
        Self { rows }
    }
}

impl App for SysInfoApp {
    fn title(&self) -> &str {
        "SISTEMA"
    }

    fn content_size(&self) -> (i32, i32) {
        (600, 218)
    }

    fn draw(&mut self, canvas: &mut Canvas, area: Rect, _focused: bool) {
        for (i, (label, value)) in self.rows.iter().enumerate() {
            let y = area.y + 18 + i as i32 * 28;
            canvas.draw_text(label, area.x + 18, y, theme::ACCENT);
            canvas.draw_text(value, area.x + VALUE_X, y, theme::TEXT);
        }
    }
}
