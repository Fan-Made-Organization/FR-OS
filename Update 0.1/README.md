# mi_os

Sistema operativo en Rust (x86_64). Se prueba en QEMU.

## Ejecutar
    cargo run

## Mapa del kernel
    kernel/src/
    ├── main.rs          punto de entrada
    ├── panic.rs         manejo de panicos
    ├── arch/            cpu (CPUID), power (reboot)
    ├── memory/          mapa de memoria (luego: frames, paginacion, heap)
    ├── drivers/         keyboard, pci (luego: mouse, timer, disco, usb...)
    ├── gfx/             framebuffer, font, console
    └── shell/           editor de linea + comandos
