//! El escritorio: coordina ventanas, barra superior, cursor y entrada.
//!
//! Dibujo por regiones: cada cambio marca la zona afectada y solo esa zona
//! se redibuja y se copia a la pantalla.
use alloc::vec::Vec;

use super::{
    anim::{self, Tween},
    topbar::{self, BarState, Layout, MenuAction},
    wallpaper,
    window::{Window, TITLE_H},
};
use crate::apps::{self, AppKind};
use crate::arch::{
    pit, power,
    rtc::{self, Time},
};
use crate::drivers::{keyboard::Key, mouse::MouseEvent};
use crate::gfx::{canvas::Canvas, cursor, rect::Rect, screen::Screen};
use crate::info::SystemInfo;
use crate::status::Status;

#[derive(Clone, Copy)]
struct Drag {
    id: u32,
    off_x: i32,
    off_y: i32,
}

pub struct Desktop {
    screen: Screen<'static>,
    canvas: Canvas,
    wallpaper: Canvas, // se dibuja una vez y se copia por regiones
    info: SystemInfo,
    status: Status,

    windows: Vec<Window>, // orden estable (el de las pastillas de la barra)
    z_order: Vec<u32>,    // ids de abajo hacia arriba
    next_id: u32,

    cursor_x: i32,
    cursor_y: i32,
    left_down: bool,
    drag: Option<Drag>,
    launcher_open: bool,
    panel_open: bool,
    launcher_anim: Option<Tween>,
    panel_anim: Option<Tween>,

    now: u64,
    last_second: u64,
    time: Time,

    scene_area: Option<Rect>,   // zona del lienzo que hay que redibujar
    present_area: Option<Rect>, // zona a copiar a la pantalla
}

fn cursor_area(x: i32, y: i32) -> Rect {
    Rect::new(x, y, cursor::WIDTH, cursor::HEIGHT)
}

fn merge(a: Option<Rect>, b: Rect) -> Option<Rect> {
    Some(match a {
        Some(p) => p.union(&b),
        None => b,
    })
}

impl Desktop {
    pub fn new(screen: Screen<'static>, info: SystemInfo) -> Self {
        let (w, h) = (screen.width(), screen.height());
        let mut d = Self {
            canvas: Canvas::new(w as usize, h as usize),
            wallpaper: wallpaper::render(w as usize, h as usize),
            screen,
            info,
            status: Status::probe(),
            windows: Vec::new(),
            z_order: Vec::new(),
            next_id: 1,
            cursor_x: w / 2,
            cursor_y: h / 2,
            left_down: false,
            drag: None,
            launcher_open: false,
            panel_open: false,
            launcher_anim: None,
            panel_anim: None,
            now: 0,
            last_second: 0,
            time: rtc::now_local(),
            scene_area: None,
            present_area: None,
        };
        d.invalidate_all();
        d
    }

    // ───────── invalidacion ─────────

    fn invalidate(&mut self, r: Rect) {
        self.present_area = merge(self.present_area, r);
    }

    fn invalidate_scene(&mut self, r: Rect) {
        self.scene_area = merge(self.scene_area, r);
        self.invalidate(r);
    }

    fn invalidate_all(&mut self) {
        let b = self.canvas.bounds();
        self.invalidate_scene(b);
    }

    pub fn needs_render(&self) -> bool {
        self.present_area.is_some()
    }

    // ───────── ventanas ─────────

    pub fn open(&mut self, kind: AppKind) {
        let app = apps::create(kind, &self.info);
        let step = (self.windows.len() % 6) as i32;
        let id = self.next_id;
        self.next_id += 1;

        self.windows
            .push(Window::new(id, app, 40 + step * 36, topbar::TOP + 24 + step * 30));
        self.z_order.push(id);
        self.invalidate_all();
    }

    fn index_of(&self, id: u32) -> Option<usize> {
        self.windows.iter().position(|w| w.id == id)
    }

    fn raise(&mut self, id: u32) {
        if self.z_order.last() == Some(&id) {
            return; // ya esta al frente
        }
        self.z_order.retain(|&z| z != id);
        self.z_order.push(id);
        self.invalidate_all();
    }

    fn close(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
        self.z_order.retain(|&z| z != id);
        self.drag = None;
        self.invalidate_all();
    }

    fn drag_window(&mut self, d: Drag) {
        let (sw, sh) = (self.canvas.width(), self.canvas.height());
        let Some(i) = self.index_of(d.id) else { return };
        let old = self.windows[i].outer_rect();

        let nx = (self.cursor_x - d.off_x).clamp(-(old.w - 80), sw - 80);
        let ny = (self.cursor_y - d.off_y).clamp(topbar::TOP, sh - TITLE_H);
        if nx == self.windows[i].rect.x && ny == self.windows[i].rect.y {
            return;
        }

        self.windows[i].rect.x = nx;
        self.windows[i].rect.y = ny;
        let new = self.windows[i].outer_rect();
        self.invalidate_scene(old.union(&new));
    }

    // ───────── entrada ─────────

    pub fn on_key(&mut self, key: Key) {
        let Some(&id) = self.z_order.last() else { return };
        let Some(i) = self.index_of(id) else { return };
        if self.windows[i].app.on_key(key) {
            let r = self.windows[i].outer_rect();
            self.invalidate_scene(r);
        }
    }

    pub fn on_mouse(&mut self, ev: MouseEvent) {
        // mover el cursor (solo se copia a pantalla, sin redibujar la escena)
        let old = cursor_area(self.cursor_x, self.cursor_y);
        self.cursor_x = (self.cursor_x + ev.dx).clamp(0, self.canvas.width() - 1);
        self.cursor_y = (self.cursor_y + ev.dy).clamp(0, self.canvas.height() - 1);
        self.invalidate(old);
        self.invalidate(cursor_area(self.cursor_x, self.cursor_y));

        // el menu resalta el item bajo el cursor
        if self.launcher_open && (ev.dx != 0 || ev.dy != 0) {
            self.invalidate_scene(topbar::menu_rect().inflate(4));
        }

        if ev.left {
            if !self.left_down {
                self.on_click();
            } else if let Some(d) = self.drag {
                self.drag_window(d);
            }
        } else {
            self.drag = None;
        }
        self.left_down = ev.left;
    }

    fn open_launcher(&mut self) {
        self.launcher_open = true;
        self.launcher_anim = Some(Tween::new(self.now, anim::DURATION));
        self.invalidate_all();
    }

    fn open_panel(&mut self) {
        self.panel_open = true;
        self.panel_anim = Some(Tween::new(self.now, anim::DURATION));
        self.status.refresh();
        self.invalidate_all();
    }

    fn on_click(&mut self) {
        let (x, y) = (self.cursor_x, self.cursor_y);
        let sw = self.canvas.width();
        let lay = Layout::compute(sw, &self.windows, self.z_order.last().copied());

        // 1) menu de apps abierto
        if self.launcher_open {
            self.launcher_open = false;
            self.launcher_anim = None;
            self.invalidate_all();
            for (i, (_, action)) in topbar::MENU_ITEMS.iter().enumerate() {
                if topbar::menu_item_rect(i).contains(x, y) {
                    self.run_action(*action);
                    return;
                }
            }
            if lay.launcher.contains(x, y) {
                return;
            }
        }

        // 2) centro de control abierto
        if self.panel_open {
            if topbar::panel_rect(sw).contains(x, y) {
                return;
            }
            self.panel_open = false;
            self.panel_anim = None;
            self.invalidate_all();
            if lay.tray.contains(x, y) {
                return;
            }
        }

        // 3) abrir menu de apps / centro de control
        if lay.launcher.contains(x, y) {
            self.open_launcher();
            return;
        }
        if lay.tray.contains(x, y) {
            self.open_panel();
            return;
        }

        // 4) pastillas de ventanas
        for (id, r) in &lay.chips {
            if r.contains(x, y) {
                self.raise(*id);
                return;
            }
        }

        // 5) ventanas, de arriba hacia abajo
        let hit = self
            .z_order
            .iter()
            .rev()
            .copied()
            .find(|&id| self.index_of(id).map_or(false, |i| self.windows[i].rect.contains(x, y)));

        let Some(id) = hit else { return };
        self.raise(id);

        let Some(i) = self.index_of(id) else { return };
        let (on_close, on_title, wx, wy) = {
            let w = &self.windows[i];
            (
                w.close_rect().contains(x, y),
                w.title_rect().contains(x, y),
                w.rect.x,
                w.rect.y,
            )
        };

        if on_close {
            self.close(id);
        } else if on_title {
            self.drag = Some(Drag { id, off_x: x - wx, off_y: y - wy });
        }
    }

    fn run_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::Open(kind) => self.open(kind),
            MenuAction::Reboot => power::reboot(),
        }
    }

    // ───────── tiempo ─────────

    /// Se llama en cada vuelta del bucle principal con los ticks actuales.
    pub fn update(&mut self, now: u64) {
        self.now = now;

        // reloj: una vez por segundo
        let second = now / pit::HZ as u64;
        if second != self.last_second {
            self.last_second = second;
            let t = rtc::now_local();
            if t.minute != self.time.minute || t.hour != self.time.hour {
                self.time = t;
                self.invalidate_scene(topbar::bar_area(self.canvas.width()));
            }
        }

        // animaciones de los desplegables
        let sw = self.canvas.width();
        if let Some(t) = self.launcher_anim {
            self.invalidate_scene(topbar::menu_rect().inflate(6));
            if t.done(now) {
                self.launcher_anim = None;
            }
        }
        if let Some(t) = self.panel_anim {
            self.invalidate_scene(topbar::panel_rect(sw).inflate(6));
            if t.done(now) {
                self.panel_anim = None;
            }
        }
    }

    // ───────── dibujo ─────────

    fn redraw_scene(&mut self, area: Rect) {
        let Some(area) = area.intersect(&self.canvas.bounds()) else { return };
        self.canvas.set_clip(area);

        self.canvas.copy_region_from(&self.wallpaper, area);

        let top = self.z_order.last().copied();
        for &id in &self.z_order {
            if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
                if win.outer_rect().intersect(&area).is_some() {
                    win.draw(&mut self.canvas, Some(id) == top);
                }
            }
        }

        let now = self.now;
        let launcher_progress = self.launcher_anim.map_or(anim::FULL, |t| t.progress(now));
        let panel_progress = self.panel_anim.map_or(anim::FULL, |t| t.progress(now));

        crate::ui::topbar::draw(
            &mut self.canvas,
            &BarState {
                windows: &self.windows,
                active: top,
                time: self.time,
                launcher_open: self.launcher_open,
                panel_open: self.panel_open,
                launcher_progress,
                panel_progress,
                status: &self.status,
                cursor: (self.cursor_x, self.cursor_y),
            },
        );

        self.canvas.reset_clip();
    }

    /// Redibuja lo que haga falta y lo manda a la pantalla.
    pub fn render(&mut self) {
        let Some(present) = self.present_area.take() else { return };
        if let Some(scene) = self.scene_area.take() {
            self.redraw_scene(scene);
        }
        self.screen.present(&self.canvas, present);
        self.screen.draw_cursor(self.cursor_x, self.cursor_y);
    }
}
