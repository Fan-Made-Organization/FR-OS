//! Cola de bytes entre las interrupciones (productor) y el bucle
//! principal (consumidor). Sin locks: solo atomicos.
use core::sync::atomic::{AtomicU16, AtomicUsize, Ordering};

const CAP: usize = 256;
const MOUSE_FLAG: u16 = 0x100;

const EMPTY: AtomicU16 = AtomicU16::new(0);
static BUF: [AtomicU16; CAP] = [EMPTY; CAP];
static HEAD: AtomicUsize = AtomicUsize::new(0); // escribe la interrupcion
static TAIL: AtomicUsize = AtomicUsize::new(0); // lee el bucle principal

pub enum Ps2Byte {
    Keyboard(u8),
    Mouse(u8),
}

fn push(value: u16) {
    let head = HEAD.load(Ordering::Relaxed);
    let next = (head + 1) % CAP;
    if next == TAIL.load(Ordering::Acquire) {
        return; // cola llena: se descarta el byte
    }
    BUF[head].store(value, Ordering::Relaxed);
    HEAD.store(next, Ordering::Release);
}

pub fn push_keyboard(byte: u8) {
    push(byte as u16);
}

pub fn push_mouse(byte: u8) {
    push(MOUSE_FLAG | byte as u16);
}

pub fn pop() -> Option<Ps2Byte> {
    let tail = TAIL.load(Ordering::Relaxed);
    if tail == HEAD.load(Ordering::Acquire) {
        return None;
    }
    let value = BUF[tail].load(Ordering::Relaxed);
    TAIL.store((tail + 1) % CAP, Ordering::Release);
    Some(if value & MOUSE_FLAG != 0 {
        Ps2Byte::Mouse(value as u8)
    } else {
        Ps2Byte::Keyboard(value as u8)
    })
}

pub fn is_empty() -> bool {
    TAIL.load(Ordering::Relaxed) == HEAD.load(Ordering::Relaxed)
}
