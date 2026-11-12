//! mi_os 4.0: punto de entrada del kernel.
//! Aqui solo se arranca; cada cosa vive en su modulo.
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod apps;
mod arch;
mod crash;
mod drivers;
mod gfx;
mod info;
mod memory;
mod panic;
mod shell;
mod status;
mod ui;

use bootloader_api::{entry_point, BootInfo};
use x86_64::instructions::interrupts;

use apps::AppKind;
use arch::pit;
use drivers::{
    input::{self, Ps2Byte},
    keyboard::Keyboard,
    mouse::Mouse,
};
use gfx::screen::Screen;
use info::SystemInfo;
use ui::desktop::Desktop;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    drivers::serial::init();
    crate::log!("mi_os 4.0: arrancando");

    memory::heap::init();
    crate::log!("heap listo");

    let usable_kib = memory::usable_kib(&boot_info.memory_regions);
    let fb = boot_info.framebuffer.as_mut().expect("sin framebuffer");
    // Desde aqui cualquier fallo muestra la pantalla de error.
    let screen = Screen::new(fb);
    let info = SystemInfo {
        usable_kib,
        screen_width: screen.width(),
        screen_height: screen.height(),
    };
    crate::log!("pantalla {}x{}", info.screen_width, info.screen_height);

    arch::gdt::init();
    arch::interrupts::init();
    arch::pic::init();
    arch::pit::init();
    drivers::ps2::init();
    crate::log!("interrupciones y PS/2 listos");

    let mut desktop = Desktop::new(screen, info);
    desktop.open(AppKind::Terminal);

    let mut keyboard = Keyboard::new();
    let mut mouse = Mouse::new();

    interrupts::enable();
    crate::log!("escritorio listo");

    loop {
        // 1) procesar todo lo que llego por interrupciones
        while let Some(byte) = input::pop() {
            match byte {
                Ps2Byte::Keyboard(code) => {
                    if let Some(key) = keyboard.feed(code) {
                        desktop.on_key(key);
                    }
                }
                Ps2Byte::Mouse(b) => {
                    if let Some(event) = mouse.feed(b) {
                        desktop.on_mouse(event);
                    }
                }
            }
        }

        // 2) animaciones, reloj y dibujo
        desktop.update(pit::ticks());
        desktop.render();

        // 3) dormir hasta la proxima interrupcion (sin gastar CPU)
        interrupts::disable();
        if input::is_empty() && !desktop.needs_render() {
            interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}
