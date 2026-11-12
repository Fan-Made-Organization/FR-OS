# mi_os 4.0

Sistema operativo en Rust (x86_64) con escritorio grafico. Se prueba en QEMU.
Estilo "La Amenaza": negro, rojo neon, franja de peligro.

## Ejecutar
    cargo run
- Haz clic dentro de la ventana de QEMU para que capture el raton (Ctrl+Alt+G lo suelta).
- Si existe /dev/kvm se activa solo (mucho mas fluido).
- El log del kernel sale en esta misma terminal (puerto serie).

## Mapa del kernel
    kernel/src/
    ├── main.rs        arranque y bucle principal (duerme con hlt)
    ├── crash.rs       pantalla de error fatal
    ├── panic.rs
    ├── info.rs        datos del sistema
    ├── arch/          cpu, gdt, interrupts (IDT), pic, pit (timer), rtc, power
    ├── memory/        mapa de memoria, heap
    ├── drivers/       ps2, input (cola), keyboard, layout (us/latam), mouse,
    │                  pci, pci_ids, serial
    ├── status/        indicadores reales: red, bluetooth, bateria, sonido, memoria
    ├── gfx/           color, rect, font, canvas (con recorte), screen, cursor, text_grid
    ├── ui/            desktop, topbar, window, anim, icons, wallpaper, theme
    ├── apps/          terminal, sysinfo, about
    └── shell/         linea con historial + autocompletado + comandos

## Cambiar el look
Todo el color esta en kernel/src/ui/theme.rs
