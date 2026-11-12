//! Runner: lanza la imagen del OS dentro de QEMU.
use std::fs::OpenOptions;
use std::process::{self, Command};

fn kvm_available() -> bool {
    OpenOptions::new().read(true).write(true).open("/dev/kvm").is_ok()
}

fn main() {
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={}", env!("BIOS_PATH")))
        .args(["-m", "512M"])
        // el kernel escribe su log por el puerto serie: se ve en esta terminal
        .args(["-serial", "stdio"]);

    if kvm_available() {
        println!("[runner] KVM activado (rapido)");
        qemu.args(["-enable-kvm", "-cpu", "host"]);
    } else {
        println!("[runner] KVM no disponible: sera mas lento (revisa /dev/kvm y el grupo 'kvm')");
    }

    let status = qemu
        .spawn()
        .expect("No se pudo iniciar QEMU. Esta instalado? (sudo pacman -S qemu-desktop)")
        .wait()
        .unwrap();
    process::exit(status.code().unwrap_or(-1));
}
