//! Temporizador PIT: un tick cada 10 ms. Es el "latido" del sistema:
//! mueve las animaciones, el reloj y despierta al kernel cuando duerme.
use core::sync::atomic::{AtomicU64, Ordering::Relaxed};
use x86_64::instructions::port::Port;

pub const HZ: u32 = 100;
const BASE_FREQ: u32 = 1_193_182;

static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let divisor = (BASE_FREQ / HZ) as u16;
    unsafe {
        Port::<u8>::new(0x43).write(0x36); // canal 0, modo 3, 16 bits
        let mut ch0 = Port::<u8>::new(0x40);
        ch0.write(divisor as u8);
        ch0.write((divisor >> 8) as u8);
    }
}

pub fn on_tick() {
    TICKS.fetch_add(1, Relaxed);
}

pub fn ticks() -> u64 {
    TICKS.load(Relaxed)
}
