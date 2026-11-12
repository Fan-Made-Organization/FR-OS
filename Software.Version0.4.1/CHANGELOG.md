# Changelog

Todos los cambios importantes de **mi_os** se documentan aqui.
Formato basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/).

---

## [4.0.1] - Correccion de arranque

### Corregido
- **Fallo de proteccion general (codigo 16) en la primera interrupcion.**
  La GDT nueva no tenia segmento de datos: el indice 2 (el que usa SS) caia
  sobre el descriptor del TSS, y al volver de la primera interrupcion la CPU
  intentaba cargar SS con un descriptor de sistema. Ahora la GDT incluye un
  segmento de datos del kernel y se cargan SS, DS y ES.

---

## [4.0.0] - La Amenaza: estilo y fluidez

Actualizacion de estilo, rendimiento y drivers. Se reescribe el bucle
principal para que trabaje por interrupciones y duerma cuando no hay nada
que hacer.

### Agregado
- **Interrupciones** (`arch/`): GDT con TSS y pila propia para el doble
  fallo, IDT con manejadores de excepciones, PIC remapeado y temporizador
  PIT a 100 Hz.
- **Pantalla de error fatal** (`crash.rs`): panicos y excepciones se dibujan
  directo en el framebuffer (sin heap) y tambien salen por el puerto serie.
- **Log por puerto serie** (`drivers/serial.rs`, macro `log!`): se ve en la
  terminal con `-serial stdio`.
- **Cola de entrada sin locks** (`drivers/input.rs`) entre las interrupciones
  y el bucle principal.
- **PS/2 por IRQ** (`drivers/ps2.rs`): inicializacion completa del controlador
  (IRQ1 e IRQ12) en lugar de polling.
- **Teclado latinoamericano** (`drivers/layout.rs`): distribucion `latam`
  (n con tilde, signos, AltGr, tilde muerta para acentos) y `us`; se cambia
  con el comando `layout`. Decodificador propio con modificadores, bloq mayus
  y teclas extendidas (flechas, suprimir, inicio, fin).
- **Aceleracion del raton**.
- **Nombres de fabricantes y clases PCI** (`drivers/pci_ids.rs`) en `lspci`.
- **Animaciones** (`ui/anim.rs`): el menu y el centro de control se
  "desenrollan" con suavizado.
- **Shell**: historial (flechas arriba/abajo), autocompletado con Tab y
  comandos nuevos `uptime`, `layout`, `about` y `crash`.
- **Recorte en el lienzo** (`Canvas::set_clip`) y `copy_region_from`.
- **Fondo nuevo**: degradado oscuro, resplandor rojo, marca de agua, lineas
  de barrido y franja de peligro abajo.
- El runner activa **KVM** automaticamente si esta disponible y redirige el
  log serie a la terminal.

### Cambiado
- **Estilo "La Amenaza"**: negro casi puro, rojo neon, titulos en mayusculas,
  esquinas menos redondeadas. Toda la paleta esta en `ui/theme.rs`.
- **Fluidez**:
  - El bucle principal duerme con `hlt` entre eventos (ya no usa el 100% de
    un nucleo).
  - Redibujo por regiones: solo se redibuja y copia la zona que cambio.
  - La copia al framebuffer es una sola copia de memoria por fila cuando el
    formato lo permite.
  - Mover el cursor ya no redibuja la escena.
- El reloj se actualiza una vez por segundo con el temporizador.
- `PciDevice::class_name` se reemplaza por `kind()` y `vendor_name()`.
- Las teclas dejan de usar el crate `pc-keyboard` (decodificador propio).

### Dependencias
- Agregado: `lazy_static` (tablas GDT/IDT).
- Eliminado: `pc-keyboard`.

---

## [0.3.0] - Barra superior y tema neon

Rediseno visual completo: todo el control del sistema queda arriba.

### Agregado
- **Barra superior con tres capsulas flotantes** (`ui/topbar.rs`):
  - Izquierda: boton `mi_os` (menu de apps) y nombre de la app activa.
  - Centro: una pastilla por ventana abierta; clic para traerla al frente.
  - Derecha: iconos de red, bluetooth y bateria, mas el reloj.
- **Centro de control**: panel desplegable con el estado de cada indicador
  (punto verde / ambar / gris + descripcion).
- **Modulo `status/`** con indicadores que reportan el estado real del hardware:
  - `network.rs`: detecta adaptadores Wi-Fi / Ethernet en el bus PCI.
  - `bluetooth.rs`: detecta controlador USB (y Bluetooth PCI si existe).
  - `battery.rs`: marcado como no disponible hasta que exista driver ACPI.
  - `sound.rs`: detecta controladores de audio en PCI.
  - `memory.rs`: uso real del heap del kernel.
- **Tema "noche urbana / rosa neon"** centralizado en `ui/theme.rs`.
- **Fondo de pantalla** (`ui/wallpaper.rs`): degradado + resplandores radiales,
  dibujado una sola vez y copiado en cada redibujo.
- **Iconos** (`ui/icons.rs`): senal de red, bluetooth y bateria.
- Primitivas de dibujo nuevas en `gfx/canvas.rs`:
  `round_rect` (esquinas redondeadas con transparencia), `radial_glow`,
  `draw_sprite`, `copy_from`, `put`.
- `Rect::inflate`.
- Campo `subclass` en `PciDevice` (necesario para distinguir Wi-Fi de Ethernet).
- `memory::heap::stats()` para consultar uso del heap.
- Ventana activa con borde rosa y barra de titulo con degradado.

### Cambiado
- La barra de tareas inferior fue reemplazada por la barra superior.
- Las ventanas ya no pueden subir por encima de la barra superior.
- El boton de cerrar ahora es una pastilla redondeada.
- Ventana "Acerca de" actualizada con las nuevas instrucciones.

### Eliminado
- `ui/taskbar.rs` (reemplazado por `ui/topbar.rs`).

---

## [0.2.0] - Escritorio

Primer escritorio grafico con raton y ventanas.

### Agregado
- **Driver de raton PS/2** (`drivers/mouse.rs`) con decodificacion de paquetes
  de 3 bytes, botones izquierdo/derecho y resincronizacion.
- **Controlador PS/2 compartido** (`drivers/ps2.rs`): separa los bytes del
  teclado y del raton, e inicializa el raton.
- **Heap** (`memory/heap.rs`): bloque fijo de 32 MiB con `linked_list_allocator`;
  habilita `Vec`, `Box` y `String`.
- **Reloj de tiempo real** (`arch/rtc.rs`): lee el CMOS, soporta BCD y modo
  12 h, zona horaria UTC-6 (Honduras) configurable.
- **Escritorio** (`ui/desktop.rs`): ventanas con orden de capas, arrastre,
  cierre, cursor propio y barra de tareas inferior con menu de inicio.
- **Ventanas** (`ui/window.rs`) con sombra, barra de titulo y boton de cierre.
- **Aplicaciones** (`apps/`) sobre un trait `App`:
  - `terminal.rs`: la terminal ahora vive dentro de una ventana.
  - `sysinfo.rs`: resumen del hardware.
  - `about.rs`: informacion del sistema.
- **Lienzo en memoria** (`gfx/canvas.rs`) y **pantalla** (`gfx/screen.rs`):
  se dibuja todo en memoria y se copia al framebuffer solo la region modificada.
- **Cursor** con sprite de flecha (`gfx/cursor.rs`).
- **Cuadricula de texto** (`gfx/text_grid.rs`) con scroll, independiente de
  las ventanas.
- Comandos nuevos del shell: `time`, `screen`.
- `info.rs`: datos del sistema compartidos (memoria, resolucion).

### Cambiado
- `Color` pasa de tupla `(u8, u8, u8)` a `u32` (`0x00RRGGBB`) con helpers
  `rgb`, `channels` y `blend` (`gfx/color.rs`).
- El shell ahora escribe sobre `TextGrid` en lugar de dibujar directo en pantalla.
- El teclado ya no lee los puertos directamente: recibe scancodes de `ps2`.
- Longitud maxima de linea del shell reducida a 70 para que no se parta.

### Eliminado
- `gfx/console.rs` y `gfx/framebuffer.rs` (reemplazados por `canvas`, `screen`
  y `text_grid`).

---

## [0.1.1] - Reorganizacion en modulos

### Cambiado
- El kernel dejo de ser un solo archivo y se dividio en modulos:
  `arch/`, `drivers/`, `gfx/`, `memory/`, `shell/` y `panic.rs`.
- `main.rs` quedo solo con el arranque.
- Las capas ya no dependen hacia arriba: `arch::cpu` devuelve datos
  (`CpuInfo`), `drivers::pci::scan` entrega cada dispositivo por callback, y el
  shell se encarga de imprimirlos.
- Cada comando del shell es una funcion en `shell/commands.rs`.

### Corregido
- `__cpuid` es `unsafe` y se llamaba sin bloque `unsafe`; ahora esta envuelto.

### Agregado
- `README.md` con el mapa del proyecto.
- `.gitignore` (ignora `/target`).

---

## [0.1.0] - Primer arranque

### Agregado
- Proyecto base: workspace con `build.rs` que genera la imagen de disco
  booteable (crate `bootloader` 0.11) y un runner que lanza QEMU.
- `rust-toolchain.toml` (nightly, `rust-src`, `llvm-tools-preview`) y
  `.cargo/config.toml` (`bindeps`).
- Kernel `no_std` / `no_main` con punto de entrada `bootloader_api`.
- Salida grafica por framebuffer con fuente bitmap (Noto Sans Mono, 16 px).
- Terminal con scroll y colores.
- Teclado PS/2 por polling con layout US.
- Comandos: `help`, `clear`, `echo`, `cpu` (CPUID), `mem` (mapa de memoria),
  `lspci` (escaneo del bus PCI), `reboot`.
- `[profile.dev] opt-level = 3` para que el dibujo de pixeles sea usable.

---

## Problemas conocidos

- **Rendimiento:** corregido en 4.0 (interrupciones, `hlt`, redibujo por
  regiones y copia por filas). Falta medirlo en hardware real.
- **Texto:** la fuente embebida solo cubre ASCII; no hay acentos ni `n` con tilde.
- **Teclado:** distribuciones `us` y `latam` (la latam se hizo de memoria;
  si alguna tecla no coincide, avisar). Los acentos dependen de que la
  fuente embebida tenga esos glifos.
- **Esquinas redondeadas** sin suavizado (anti-aliasing).
- **Heap** de tamano fijo (32 MiB) dentro de la imagen del kernel.
- **Hardware sin driver:** red, bluetooth, audio y bateria se detectan pero no
  funcionan; USB aun no existe.
- **Arranque:** solo BIOS probado en QEMU; no se ha probado en hardware real ni UEFI.
- **Excepciones:** ahora hay pantalla de error, pero no hay recuperacion.

## Proximos pasos

1. Paginacion y heap real.
2. Pila USB (xHCI) para teclado, raton y memorias.
3. Archivos: disco (AHCI/NVMe) y sistema de archivos.
4. Mas aplicaciones y generacion del archivo `.img`.
