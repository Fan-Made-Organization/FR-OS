//! Reloj de tiempo real (CMOS).
use x86_64::instructions::port::Port;

/// Honduras es UTC-6. Cambialo si estas en otra zona.
const UTC_OFFSET_HOURS: i32 = -6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

fn read_reg(reg: u8) -> u8 {
    unsafe {
        Port::<u8>::new(0x70).write(reg);
        Port::<u8>::new(0x71).read()
    }
}

fn bcd_to_bin(v: u8) -> u8 {
    (v & 0x0F) + (v >> 4) * 10
}

pub fn now_local() -> Time {
    // Esperar a que el RTC no este actualizandose.
    for _ in 0..10_000 {
        if read_reg(0x0A) & 0x80 == 0 {
            break;
        }
    }

    let mut second = read_reg(0x00);
    let mut minute = read_reg(0x02);
    let raw_hour = read_reg(0x04);
    let status_b = read_reg(0x0B);

    let pm = raw_hour & 0x80 != 0;
    let mut hour = raw_hour & 0x7F;

    if status_b & 0x04 == 0 {
        // Modo BCD
        second = bcd_to_bin(second);
        minute = bcd_to_bin(minute);
        hour = bcd_to_bin(hour);
    }
    if status_b & 0x02 == 0 {
        // Modo 12 horas
        hour = hour % 12 + if pm { 12 } else { 0 };
    }

    let local = (hour as i32 + UTC_OFFSET_HOURS).rem_euclid(24) as u8;
    Time { hour: local, minute, second }
}
