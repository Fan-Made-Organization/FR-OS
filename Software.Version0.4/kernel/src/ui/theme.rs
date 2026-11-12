//! Paleta de colores. Cambia aqui para cambiar todo el look.
//! Tema "LA AMENAZA": negro casi puro, rojo sangre/neon, tension.
use crate::gfx::color::{rgb, Color};

// fondo
pub const DESKTOP_TOP: Color = rgb(5, 3, 8);
pub const DESKTOP_BOTTOM: Color = rgb(34, 4, 12);
pub const GLOW_A: Color = rgb(230, 20, 55); // resplandor principal
pub const GLOW_B: Color = rgb(110, 20, 140); // resplandor secundario
pub const HAZARD: Color = rgb(190, 16, 40); // franja de peligro
pub const HAZARD_DARK: Color = rgb(8, 4, 10);

// marca
pub const HOT: Color = rgb(235, 25, 60);
pub const HOT_SOFT: Color = rgb(255, 95, 115);
pub const ON_HOT: Color = rgb(18, 2, 6);

// barra y paneles
pub const BAR_BG: Color = rgb(9, 5, 12);
pub const PANEL_BG: Color = rgb(13, 7, 16);
pub const MENU_HOVER: Color = rgb(150, 16, 40);
pub const CHIP_ACTIVE: Color = rgb(120, 14, 36);
pub const CHIP_IDLE: Color = rgb(34, 22, 38);

// ventanas
pub const TITLE_FOCUSED_TOP: Color = rgb(175, 18, 44);
pub const TITLE_FOCUSED_BOTTOM: Color = rgb(98, 10, 28);
pub const TITLE_IDLE: Color = rgb(38, 28, 42);
pub const TITLE_TEXT: Color = rgb(250, 238, 240);
pub const WINDOW_BG: Color = rgb(16, 9, 20);
pub const WINDOW_BORDER: Color = rgb(10, 5, 12);
pub const SHADOW: Color = rgb(0, 0, 6);

// texto
pub const TEXT: Color = rgb(240, 234, 238);
pub const MUTED: Color = rgb(150, 130, 140);
pub const ACCENT: Color = HOT_SOFT;
pub const ERROR: Color = rgb(255, 170, 70);

// estados
pub const OK: Color = rgb(120, 230, 165);
pub const AMBER: Color = rgb(255, 196, 90);

// terminal
pub const TERM_BG: Color = rgb(7, 4, 10);
pub const TERM_FG: Color = rgb(236, 228, 232);

// forma
pub const RADIUS: i32 = 9;
