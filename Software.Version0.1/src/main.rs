//! Runner: lanza la imagen del OS dentro de QEMU.
use std::process::{self, Command};

fn main() {
    let status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={}", env!("BIOS_PATH")))
        .arg("-m")
        .arg("512M")
        .spawn()
        .expect("No se pudo iniciar QEMU. Esta instalado? (sudo pacman -S qemu-desktop)")
        .wait()
        .unwrap();
    process::exit(status.code().unwrap_or(-1));
}
