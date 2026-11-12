//! Que hacer cuando el kernel entra en panico.
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: mostrar el mensaje en pantalla
    loop {
        core::hint::spin_loop();
    }
}
