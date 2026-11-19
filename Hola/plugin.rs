//! Plugin de ejemplo: agrega el comando `hola`.
//! Es el plugin mas pequeno posible: sirve para comprobar que la instalacion
//! funciona y como punto de partida.
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "hola",
    version: "1.0.0",
    author: "FR-OS",
    description: "Comando de ejemplo: hola [nombre]",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.command("hola", "saluda: hola [nombre]", hola);
    api.log("listo");
}

fn hola(out: &mut dyn Write, args: &str) {
    let quien = if args.is_empty() { "mundo" } else { args };
    let _ = writeln!(out, "Hola, {}! Este mensaje viene de un plugin.", quien);
}
