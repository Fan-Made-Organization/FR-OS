//! mi_os: punto de entrada del kernel.
//! Aqui solo se arranca; cada cosa vive en su modulo.
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod gfx;
mod memory;
mod panic;
mod shell;

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;

use drivers::keyboard::Keyboard;
use gfx::{console::Terminal, framebuffer::Screen, ACCENT, FG};
use shell::{Shell, SystemInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let info = SystemInfo {
        usable_kib: memory::usable_kib(&boot_info.memory_regions),
    };

    let fb = boot_info.framebuffer.as_mut().expect("sin framebuffer");
    let mut term = Terminal::new(Screen::new(fb));

    term.set_color(ACCENT);
    let _ = writeln!(term, "=== mi_os v0.1 ===");
    term.set_color(FG);
    let _ = writeln!(term, "Escribe 'help' para ver los comandos.\n");

    let mut shell = Shell::new();
    shell.show_prompt(&mut term);

    let mut keyboard = Keyboard::new();
    loop {
        match keyboard.poll() {
            Some(key) => shell.handle_key(&mut term, key, &info),
            None => core::hint::spin_loop(),
        }
    }
}
