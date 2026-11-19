//! Colores de acento extra. Se eligen con `accent <nombre>`.
//! Los tonos de camuflaje (oliva, ocre, cobre) vienen de la portada de
//! "Lindo y Bello".
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "temas-extra",
    version: "1.0.0",
    author: "FR-OS",
    description: "6 colores de acento extra: plata, oliva, ocre, cobre, sangre, hielo",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.accent("plata", rgb(190, 190, 205));
    api.accent("oliva", rgb(130, 140, 60));
    api.accent("ocre", rgb(205, 150, 45));
    api.accent("cobre", rgb(200, 100, 60));
    api.accent("sangre", rgb(185, 20, 35));
    api.accent("hielo", rgb(150, 210, 240));
}
