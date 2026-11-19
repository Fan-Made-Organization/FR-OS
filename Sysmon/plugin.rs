//! Monitor del sistema: un widget "MEM 12%" en la barra superior y el
//! comando `sysmon` con el uso de memoria y el tiempo encendido.
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "sysmon",
    version: "1.0.0",
    author: "FR-OS",
    description: "Widget de memoria en la barra y comando sysmon",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.widget(memoria);
    api.command("sysmon", "uso de memoria y tiempo encendido", sysmon);
}

/// Texto del widget: porcentaje del heap en uso.
fn memoria() -> String {
    let (usado, total) = heap_usage();
    format!("MEM {}%", usado * 100 / total.max(1))
}

fn sysmon(out: &mut dyn Write, _args: &str) {
    let (usado, total) = heap_usage();
    let s = uptime_seconds();
    let _ = writeln!(out, "Heap: {} KiB usados de {} MiB", usado / 1024, total / 1024 / 1024);
    let _ = writeln!(out, "Encendido: {}h {}m {}s", s / 3600, (s / 60) % 60, s % 60);
}
