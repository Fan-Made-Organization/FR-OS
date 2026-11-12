//! Controlador de interrupciones 8259 (PIC).
use x86_64::instructions::port::Port;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const OFFSET1: u8 = 32; // IRQ 0-7  -> vectores 32-39
const OFFSET2: u8 = 40; // IRQ 8-15 -> vectores 40-47

pub const TIMER_VECTOR: u8 = OFFSET1; // IRQ0
pub const KEYBOARD_VECTOR: u8 = OFFSET1 + 1; // IRQ1
pub const MOUSE_VECTOR: u8 = OFFSET2 + 4; // IRQ12

fn io_wait() {
    unsafe { Port::<u8>::new(0x80).write(0) };
}

/// Reubica las IRQ (para no chocar con las excepciones de la CPU) y deja
/// activas solo timer, teclado y raton.
pub fn init() {
    unsafe {
        let mut c1 = Port::<u8>::new(PIC1_CMD);
        let mut d1 = Port::<u8>::new(PIC1_DATA);
        let mut c2 = Port::<u8>::new(PIC2_CMD);
        let mut d2 = Port::<u8>::new(PIC2_DATA);

        c1.write(0x11);
        io_wait();
        c2.write(0x11);
        io_wait();
        d1.write(OFFSET1);
        io_wait();
        d2.write(OFFSET2);
        io_wait();
        d1.write(4); // el esclavo cuelga de la IRQ2
        io_wait();
        d2.write(2);
        io_wait();
        d1.write(0x01);
        io_wait();
        d2.write(0x01);
        io_wait();

        // 0 = activa. Maestro: IRQ0, IRQ1, IRQ2. Esclavo: IRQ12.
        d1.write(0b1111_1000);
        d2.write(0b1110_1111);
    }
}

/// Avisa al PIC que terminamos de atender la IRQ.
pub fn eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            Port::<u8>::new(PIC2_CMD).write(0x20);
        }
        Port::<u8>::new(PIC1_CMD).write(0x20);
    }
}
