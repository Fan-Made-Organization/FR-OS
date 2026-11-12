//! Implementacion de los comandos del shell.
use core::fmt::Write;

use super::SystemInfo;
use crate::arch::{cpu::CpuInfo, power};
use crate::drivers::pci;
use crate::gfx::{console::Terminal, ERROR, FG};

pub fn execute(t: &mut Terminal, line: &str, info: &SystemInfo) {
    let line = line.trim();
    if line.is_empty() {
        return;
    }
    let (cmd, args) = line.split_once(' ').map_or((line, ""), |(c, a)| (c, a.trim()));

    match cmd {
        "help" => help(t),
        "clear" => t.clear(),
        "echo" => {
            let _ = writeln!(t, "{}", args);
        }
        "cpu" => cpu(t),
        "mem" => {
            let _ = writeln!(t, "Memoria utilizable: {} MiB", info.usable_kib / 1024);
        }
        "lspci" => lspci(t),
        "reboot" => power::reboot(),
        _ => {
            t.set_color(ERROR);
            let _ = writeln!(t, "comando no encontrado: {}", cmd);
            t.set_color(FG);
        }
    }
}

fn help(t: &mut Terminal) {
    let _ = writeln!(t, "Comandos: help, clear, echo <texto>, cpu, mem, lspci, reboot");
}

fn cpu(t: &mut Terminal) {
    let cpu = CpuInfo::read();
    let _ = writeln!(t, "Fabricante: {}", cpu.vendor());
    let _ = writeln!(t, "Modelo:     {}", cpu.brand());
}

fn lspci(t: &mut Terminal) {
    pci::scan(|d| {
        let _ = writeln!(
            t,
            "{:02x}:{:02x}.{} {:04x}:{:04x} {}",
            d.bus, d.slot, d.func, d.vendor, d.device, d.class_name()
        );
    });
}
