//! Implementacion de los comandos del shell.
use core::fmt::Write;

use crate::arch::{cpu::CpuInfo, pit, power, rtc};
use crate::drivers::{
    keyboard,
    layout::Layout,
    pci,
};
use crate::gfx::text_grid::TextGrid;
use crate::info::SystemInfo;
use crate::ui::theme;

pub const COMMANDS: [&str; 13] = [
    "help", "clear", "echo", "cpu", "mem", "screen", "time", "uptime", "lspci", "layout",
    "crash", "reboot", "about",
];

pub fn execute(g: &mut TextGrid, line: &str, info: &SystemInfo) {
    let line = line.trim();
    if line.is_empty() {
        return;
    }
    let (cmd, args) = line.split_once(' ').map_or((line, ""), |(c, a)| (c, a.trim()));

    match cmd {
        "help" => help(g),
        "clear" => g.clear(),
        "echo" => {
            let _ = writeln!(g, "{}", args);
        }
        "cpu" => cpu(g),
        "mem" => {
            let _ = writeln!(g, "Memoria utilizable: {} MiB", info.usable_kib / 1024);
        }
        "screen" => {
            let _ = writeln!(g, "Pantalla: {}x{}", info.screen_width, info.screen_height);
        }
        "time" => {
            let t = rtc::now_local();
            let _ = writeln!(g, "{:02}:{:02}:{:02}", t.hour, t.minute, t.second);
        }
        "uptime" => uptime(g),
        "lspci" => lspci(g),
        "layout" => layout(g, args),
        "about" => {
            let _ = writeln!(g, "MI_OS 4.0 - kernel x86_64 escrito en Rust");
        }
        "crash" => panic!("prueba de la pantalla de error (comando 'crash')"),
        "reboot" => power::reboot(),
        _ => {
            g.set_color(theme::ERROR);
            let _ = writeln!(g, "comando no encontrado: {}", cmd);
            g.reset_color();
        }
    }
}

fn help(g: &mut TextGrid) {
    let _ = writeln!(g, "help      esta ayuda          clear     limpiar pantalla");
    let _ = writeln!(g, "echo X    repetir texto       cpu       procesador");
    let _ = writeln!(g, "mem       memoria             screen    resolucion");
    let _ = writeln!(g, "time      hora                uptime    tiempo encendido");
    let _ = writeln!(g, "lspci     dispositivos PCI    layout    teclado (us/latam)");
    let _ = writeln!(g, "crash     probar pantalla de error      reboot    reiniciar");
}

fn cpu(g: &mut TextGrid) {
    let cpu = CpuInfo::read();
    let _ = writeln!(g, "Fabricante: {}", cpu.vendor());
    let _ = writeln!(g, "Modelo:     {}", cpu.brand());
}

fn uptime(g: &mut TextGrid) {
    let s = pit::ticks() / pit::HZ as u64;
    let _ = writeln!(g, "{}h {}m {}s", s / 3600, (s / 60) % 60, s % 60);
}

fn layout(g: &mut TextGrid, args: &str) {
    match args {
        "us" => keyboard::set_layout(Layout::Us),
        "latam" => keyboard::set_layout(Layout::Latam),
        "" => {}
        _ => {
            let _ = writeln!(g, "uso: layout us | layout latam");
            return;
        }
    }
    let _ = writeln!(g, "Teclado: {}", keyboard::layout().name());
}

fn lspci(g: &mut TextGrid) {
    pci::scan(|d| {
        let _ = writeln!(
            g,
            "{:02x}:{:02x}.{} {:04x}:{:04x} {} {}",
            d.bus,
            d.slot,
            d.func,
            d.vendor,
            d.device,
            d.vendor_name(),
            d.kind()
        );
    });
}
