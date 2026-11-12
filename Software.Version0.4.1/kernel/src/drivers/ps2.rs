//! Controlador PS/2: configura teclado y raton para trabajar por IRQ.
use x86_64::instructions::port::Port;

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64; // lectura = estado, escritura = comando
const SPIN: u32 = 100_000;

fn status() -> u8 {
    unsafe { Port::<u8>::new(STATUS_PORT).read() }
}

fn read_data() -> u8 {
    unsafe { Port::<u8>::new(DATA_PORT).read() }
}

fn wait_can_write() -> bool {
    (0..SPIN).any(|_| status() & 0x02 == 0)
}

fn wait_can_read() -> bool {
    (0..SPIN).any(|_| status() & 0x01 != 0)
}

fn send_command(cmd: u8) {
    if wait_can_write() {
        unsafe { Port::<u8>::new(STATUS_PORT).write(cmd) };
    }
}

fn send_data(data: u8) {
    if wait_can_write() {
        unsafe { Port::<u8>::new(DATA_PORT).write(data) };
    }
}

fn receive() -> Option<u8> {
    wait_can_read().then(read_data)
}

fn flush() {
    for _ in 0..1000 {
        if status() & 0x01 == 0 {
            break;
        }
        read_data();
    }
}

/// Manda un byte al raton y espera su ACK (0xFA).
fn mouse_command(byte: u8) -> bool {
    send_command(0xD4);
    send_data(byte);
    receive() == Some(0xFA)
}

/// Deja listo el controlador. Llamar ANTES de habilitar interrupciones.
pub fn init() {
    // apagar los dos puertos mientras configuramos
    send_command(0xAD);
    send_command(0xA7);
    flush();

    // leer config: activar IRQ1 (bit 0) e IRQ12 (bit 1), reloj de ambos puertos
    send_command(0x20);
    let config = receive().unwrap_or(0x47);
    send_command(0x60);
    send_data((config | 0b0000_0011) & !0b0011_0000);

    // encender de nuevo
    send_command(0xAE);
    send_command(0xA8);

    // teclado: empezar a mandar teclas
    send_data(0xF4);
    let _ = receive();

    // raton: valores por defecto y empezar a mandar movimiento
    let defaults = mouse_command(0xF6);
    let streaming = mouse_command(0xF4);
    crate::log!("ps2: raton defaults={} streaming={}", defaults, streaming);

    flush();
}
