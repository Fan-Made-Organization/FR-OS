//! Aplicaciones que corren dentro de ventanas.
mod about;
mod sysinfo;
mod terminal;

use alloc::boxed::Box;

use crate::drivers::keyboard::Key;
use crate::gfx::{canvas::Canvas, rect::Rect};
use crate::info::SystemInfo;

#[derive(Clone, Copy)]
pub enum AppKind {
    Terminal,
    System,
    About,
}

/// Todo lo que una ventana necesita saber de su aplicacion.
pub trait App {
    fn title(&self) -> &str;
    /// Tamano del area de contenido (sin barra de titulo).
    fn content_size(&self) -> (i32, i32);
    fn draw(&mut self, canvas: &mut Canvas, area: Rect, focused: bool);
    /// Devuelve true si hay que redibujar.
    fn on_key(&mut self, _key: Key) -> bool {
        false
    }
}

pub fn create(kind: AppKind, info: &SystemInfo) -> Box<dyn App> {
    match kind {
        AppKind::Terminal => Box::new(terminal::TerminalApp::new(info)),
        AppKind::System => Box::new(sysinfo::SysInfoApp::new(info)),
        AppKind::About => Box::new(about::AboutApp),
    }
}
