//! Barra superior: tres capsulas flotantes.
//!   izquierda: boton MI_OS (menu de apps) + nombre de la app activa
//!   centro:    una pastilla por ventana abierta
//!   derecha:   red, bluetooth, bateria, reloj -> abre el centro de control
use alloc::{format, vec::Vec};

use super::{anim, icons, theme, window::Window};
use crate::apps::AppKind;
use crate::arch::rtc::Time;
use crate::gfx::{canvas::Canvas, color::Color, font, rect::Rect};
use crate::status::{Level, Status};

pub const BAR_Y: i32 = 8;
pub const BAR_H: i32 = 32;
/// Espacio que ocupa la barra; las ventanas no pueden subir mas.
pub const TOP: i32 = BAR_Y + BAR_H + 8;

const MARGIN: i32 = 12;
const TRAY_W: i32 = 166;

const MENU_W: i32 = 210;
const ITEM_H: i32 = 32;
const MENU_PAD: i32 = 6;

const PANEL_W: i32 = 470;
const PANEL_HEADER: i32 = 44;
const ROW_H: i32 = 48;

#[derive(Clone, Copy)]
pub enum MenuAction {
    Open(AppKind),
    Reboot,
}

pub const MENU_ITEMS: [(&str, MenuAction); 4] = [
    ("TERMINAL", MenuAction::Open(AppKind::Terminal)),
    ("SISTEMA", MenuAction::Open(AppKind::System)),
    ("ACERCA DE", MenuAction::Open(AppKind::About)),
    ("REINICIAR", MenuAction::Reboot),
];

pub struct BarState<'a> {
    pub windows: &'a [Window],
    pub active: Option<u32>,
    pub time: Time,
    pub launcher_open: bool,
    pub panel_open: bool,
    /// Progreso de la animacion de apertura (0..=anim::FULL).
    pub launcher_progress: u32,
    pub panel_progress: u32,
    pub status: &'a Status,
    pub cursor: (i32, i32),
}

// ───────── geometria (se usa para dibujar y para detectar clics) ─────────

pub struct Layout {
    pub left: Rect,
    pub launcher: Rect,
    pub title_x: i32,
    pub chips_bar: Option<Rect>,
    pub chips: Vec<(u32, Rect)>,
    pub tray: Rect,
}

impl Layout {
    pub fn compute(sw: i32, windows: &[Window], active: Option<u32>) -> Self {
        let cw = font::char_width();

        let active_title = active
            .and_then(|id| windows.iter().find(|w| w.id == id))
            .map(|w| w.title())
            .unwrap_or("ESCRITORIO");

        let launcher_w = 5 * cw + 24;
        let title_w = active_title.chars().count() as i32 * cw;
        let left_w = 6 + launcher_w + 12 + title_w + 16;
        let left = Rect::new(MARGIN, BAR_Y, left_w, BAR_H);
        let launcher = Rect::new(left.x + 6, left.y + 4, launcher_w, BAR_H - 8);

        let start = left.right() + 10;
        let mut x = start + 8;
        let mut chips = Vec::new();
        for w in windows {
            let width = w.title().chars().count() as i32 * cw + 24;
            chips.push((w.id, Rect::new(x, BAR_Y + 4, width, BAR_H - 8)));
            x += width + 6;
        }
        let chips_bar = if chips.is_empty() {
            None
        } else {
            Some(Rect::new(start, BAR_Y, x + 2 - start, BAR_H))
        };

        Self {
            left,
            launcher,
            title_x: launcher.right() + 12,
            chips_bar,
            chips,
            tray: Rect::new(sw - MARGIN - TRAY_W, BAR_Y, TRAY_W, BAR_H),
        }
    }
}

pub fn menu_rect() -> Rect {
    let h = MENU_ITEMS.len() as i32 * ITEM_H + 2 * MENU_PAD;
    Rect::new(MARGIN, TOP, MENU_W, h)
}

pub fn menu_item_rect(index: usize) -> Rect {
    let m = menu_rect();
    Rect::new(m.x + MENU_PAD, m.y + MENU_PAD + index as i32 * ITEM_H, m.w - 2 * MENU_PAD, ITEM_H)
}

pub fn panel_rect(sw: i32) -> Rect {
    Rect::new(sw - MARGIN - PANEL_W, TOP, PANEL_W, PANEL_HEADER + 5 * ROW_H + 10)
}

pub fn bar_area(sw: i32) -> Rect {
    Rect::new(0, 0, sw, TOP)
}

// ───────── dibujo ─────────

fn level_color(l: Level) -> Color {
    match l {
        Level::Ok => theme::OK,
        Level::Warn => theme::AMBER,
        Level::Off => theme::MUTED,
    }
}

fn level_word(l: Level) -> &'static str {
    match l {
        Level::Ok => "Activo",
        Level::Warn => "Detectado",
        Level::Off => "No disponible",
    }
}

/// Capsula flotante con un resplandor rojo alrededor.
fn capsule(canvas: &mut Canvas, r: Rect) {
    canvas.round_rect(r.inflate(2), theme::RADIUS + 2, theme::HOT, 46);
    canvas.round_rect(r, theme::RADIUS, theme::BAR_BG, 232);
}

/// Dibuja algo "desenrollandolo" de arriba hacia abajo segun el progreso.
fn with_reveal(canvas: &mut Canvas, full: Rect, progress: u32, draw: impl FnOnce(&mut Canvas)) {
    let saved = canvas.clip();
    let visible = Rect::new(
        full.x - 4,
        full.y - 4,
        full.w + 8,
        (full.h + 8) * progress as i32 / anim::FULL as i32,
    );
    if let Some(c) = saved.intersect(&visible) {
        canvas.set_clip(c);
        draw(canvas);
    }
    canvas.set_clip(saved);
}

pub fn draw(canvas: &mut Canvas, st: &BarState) {
    let sw = canvas.width();
    let lay = Layout::compute(sw, st.windows, st.active);
    let text_y = BAR_Y + (BAR_H - font::CHAR_HEIGHT) / 2;

    // ── izquierda ──
    capsule(canvas, lay.left);
    let chip = if st.launcher_open { theme::HOT_SOFT } else { theme::HOT };
    canvas.round_rect(lay.launcher, theme::RADIUS - 2, chip, 255);
    canvas.draw_text_centered("MI_OS", lay.launcher, theme::ON_HOT);

    let active_title = st
        .active
        .and_then(|id| st.windows.iter().find(|w| w.id == id))
        .map(|w| w.title())
        .unwrap_or("ESCRITORIO");
    canvas.draw_text(active_title, lay.title_x, text_y, theme::TEXT);

    // ── centro: ventanas abiertas ──
    if let Some(bar) = lay.chips_bar {
        capsule(canvas, bar);
        for (id, r) in &lay.chips {
            let color = if Some(*id) == st.active { theme::CHIP_ACTIVE } else { theme::CHIP_IDLE };
            canvas.round_rect(*r, theme::RADIUS - 2, color, 255);
            if let Some(w) = st.windows.iter().find(|w| w.id == *id) {
                canvas.draw_text_centered(w.title(), *r, theme::TEXT);
            }
        }
    }

    // ── derecha: estado + reloj ──
    capsule(canvas, lay.tray);
    let mut x = lay.tray.x + 16;
    let cy = BAR_Y + BAR_H / 2;

    icons::network(canvas, x, cy - icons::NETWORK_H / 2, level_color(st.status.network.level));
    x += icons::NETWORK_W + 14;
    icons::bluetooth(canvas, x, cy - icons::BLUETOOTH_H / 2, level_color(st.status.bluetooth.level));
    x += icons::BLUETOOTH_W + 14;
    icons::battery(
        canvas,
        x,
        cy - icons::BATTERY_H / 2,
        level_color(st.status.battery.level),
        None, // se llenara cuando exista el driver ACPI
    );
    x += icons::BATTERY_W + 14;
    let clock = format!("{:02}:{:02}", st.time.hour, st.time.minute);
    canvas.draw_text(&clock, x, text_y, theme::TEXT);

    // ── desplegables ──
    if st.launcher_open {
        let cursor = st.cursor;
        with_reveal(canvas, menu_rect(), st.launcher_progress, |c| draw_menu(c, cursor));
    }
    if st.panel_open {
        let status = st.status;
        with_reveal(canvas, panel_rect(sw), st.panel_progress, |c| draw_panel(c, status));
    }
}

fn draw_menu(canvas: &mut Canvas, cursor: (i32, i32)) {
    let m = menu_rect();
    canvas.round_rect(m.inflate(2), theme::RADIUS + 2, theme::HOT, 46);
    canvas.round_rect(m, theme::RADIUS, theme::PANEL_BG, 246);

    for (i, (label, _)) in MENU_ITEMS.iter().enumerate() {
        let r = menu_item_rect(i);
        if r.contains(cursor.0, cursor.1) {
            canvas.round_rect(r, theme::RADIUS - 3, theme::MENU_HOVER, 255);
        }
        canvas.draw_text(label, r.x + 14, r.y + (r.h - font::CHAR_HEIGHT) / 2, theme::TEXT);
    }
}

fn draw_panel(canvas: &mut Canvas, status: &Status) {
    let p = panel_rect(canvas.width());
    canvas.round_rect(p.inflate(2), theme::RADIUS + 2, theme::HOT, 46);
    canvas.round_rect(p, theme::RADIUS, theme::PANEL_BG, 246);

    canvas.draw_text("CENTRO DE CONTROL", p.x + 18, p.y + 14, theme::HOT_SOFT);

    for (i, ind) in status.all().iter().enumerate() {
        let y = p.y + PANEL_HEADER + i as i32 * ROW_H;
        let color = level_color(ind.level);

        // separador fino
        canvas.fill_rect(Rect::new(p.x + 18, y, p.w - 36, 1), theme::CHIP_IDLE);

        canvas.round_rect(Rect::new(p.x + 18, y + 11, 10, 10), 5, color, 255);
        canvas.draw_text(ind.label, p.x + 40, y + 6, theme::TEXT);
        canvas.draw_text(&ind.detail, p.x + 40, y + 26, theme::MUTED);

        let word = level_word(ind.level);
        let tw = canvas.text_width(word);
        canvas.draw_text(word, p.right() - 18 - tw, y + 6, color);
    }
}
