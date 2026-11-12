//! Reinicio y (mas adelante) apagado.
use x86_64::instructions::port::Port;

pub fn reboot() -> ! {
    // El controlador PS/2 puede pulsar la linea de reset de la CPU.
    unsafe { Port::<u8>::new(0x64).write(0xFE) };
    loop {
        core::hint::spin_loop();
    }
}
