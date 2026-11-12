//! Datos del sistema recogidos al arrancar.

#[derive(Clone, Copy)]
pub struct SystemInfo {
    pub usable_kib: u64,
    pub screen_width: i32,
    pub screen_height: i32,
}
