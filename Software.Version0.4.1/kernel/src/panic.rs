//! Que hacer cuando el kernel entra en panico.
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::crash::show("PANICO DEL KERNEL", format_args!("{}", info))
}
