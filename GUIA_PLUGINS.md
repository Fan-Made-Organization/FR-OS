# Guia de plugins de FR-OS

Un **plugin** es un paquete opcional que agrega cosas a FR-OS: apps en el
lanzador, comandos en la terminal, textos en la barra superior o colores de
acento. Instalarlo es **copiar una carpeta** y volver a ejecutar `cargo run`.
No hay que editar ningun archivo del sistema.

- [Instalar plugins](#1-instalar-plugins)
- [Comprobar que funcionan](#2-comprobar-que-funcionan)
- [Quitar o desactivar plugins](#3-quitar-o-desactivar-plugins)
- [Si algo falla](#4-si-algo-falla)
- [Seguridad: leelo antes de instalar](#5-seguridad)
- [Crear tu propio plugin](#6-crear-tu-propio-plugin)
- [Referencia de la API](#7-referencia-de-la-api)
- [Publicar en la rama Plugins](#8-publicar-en-la-rama-plugins)
- [Limites actuales](#9-limites-actuales)

---

## 1. Instalar plugins

Los plugins opcionales estan en la rama **Plugins** del repositorio
(catalogo en `plugins/CATALOGO.md`). Elige una forma:

### Con git (recomendado)

Desde la carpeta de tu FR-OS:

```bash
git fetch origin Plugins

# un plugin
git checkout origin/Plugins -- plugins/calculadora

# varios
git checkout origin/Plugins -- plugins/calculadora plugins/notas plugins/viborita

# TODOS los de la rama
git checkout origin/Plugins -- plugins

cargo run
```

### Sin git

1. Descarga la rama **Plugins** como ZIP desde GitHub (boton *Code* >
   *Download ZIP*, con la rama Plugins elegida).
2. Dentro del ZIP, abre la carpeta `plugins/`.
3. **Copia solo las carpetas que quieras** dentro de la carpeta `plugins/` de tu
   FR-OS. Cada plugin es una carpeta con un `plugin.rs` adentro:

```
mi_os/
└── plugins/
    ├── calculadora/
    │   └── plugin.rs
    └── notas/
        └── plugin.rs
```

4. Ejecuta `cargo run`.

> Para aplicar los cambios hay que **volver a ejecutar `cargo run`** (es el
> "reinicio"). La primera vez tarda un poco mas porque recompila el kernel.

---

## 2. Comprobar que funcionan

- Durante la compilacion, `cargo` muestra una linea por plugin:
  `plugin encontrado: calculadora`.
- Dentro de FR-OS, en la terminal, escribe **`plugins`**: lista lo instalado,
  con version, autor y lo que aporta cada uno.
- **Apps**: abre el lanzador (tecla **Win** o **F1**). Las apps de plugins
  aparecen junto a las del sistema.
- **Comandos**: escribe `help`; los de plugins salen en "De plugins:".
  El autocompletado con Tab tambien los conoce.
- **Widgets**: aparecen en la barra superior, a la izquierda de los iconos.
- **Colores**: `accent` lista todos los colores, incluidos los de plugins.

---

## 3. Quitar o desactivar plugins

| Quiero... | Hago esto |
|---|---|
| Quitar un plugin | Borra su carpeta de `plugins/` y ejecuta `cargo run` |
| Desactivarlo sin borrarlo | Renombra la carpeta con un `_` delante (`_notas`) |
| Arrancar sin NINGUN plugin | `FR_OS_NO_PLUGINS=1 cargo run` |

Las carpetas que empiezan con `_` o `.` se ignoran siempre.

---

## 4. Si algo falla

**La compilacion falla y el error menciona `plugins/algo/plugin.rs`.**
Ese plugin tiene un error o no es compatible con tu version. Quitalo
(renombralo con `_`) y vuelve a compilar. Si quieres, abre un issue con el
mensaje completo.

**Arranca en modo seguro?** Usa `FR_OS_NO_PLUGINS=1 cargo run` para confirmar
que el sistema funciona sin plugins.

**`plugins` dice `[NO] ... incompatible`.** El plugin esta hecho para otra
version de la API. FR-OS lo carga pero no lo activa. Busca una version nueva
del plugin.

**El sistema se cae con la pantalla "LA AMENAZA GANO".** Probablemente un
plugin provoco un error. El mensaje y el log por serie (`cargo run` muestra
la terminal de QEMU) dicen donde fue. Quita el ultimo plugin que agregaste.

**Un plugin no aparece.** Revisa que la carpeta tenga un archivo llamado
exactamente `plugin.rs` y que el nombre de la carpeta no empiece con `_`.

---

## 5. Seguridad

> **Un plugin es codigo que corre DENTRO del kernel, con todos los
> permisos.** FR-OS todavia no tiene aislamiento entre programas.

Un plugin malicioso o con errores podria colgar el sistema, leer o escribir
cualquier memoria o tocar el hardware directamente. Por eso:

- Instala solo plugins de fuentes en las que confies.
- **Lee el codigo** (`plugin.rs`) antes de instalarlo. Si no entiendes algo,
  no lo instales.
- Prueba los plugins nuevos en una maquina virtual (QEMU), no en hardware
  con datos importantes.
- Los plugins de la rama oficial se revisan antes de aceptarse, pero eso no
  reemplaza que tu los revises.

---

## 6. Crear tu propio plugin

### Paso a paso

1. Copia la plantilla: `plugins/_template` -> `plugins/mi_plugin`
   (el nombre en minusculas, sin espacios, **sin** guion bajo al inicio).
2. Edita `plugins/mi_plugin/plugin.rs`.
3. Ejecuta `cargo run`.

### Ejemplo minimo

```rust
use crate::plugins::api::*;

pub const INFO: PluginInfo = PluginInfo {
    name: "hola",
    version: "1.0.0",
    author: "tu nombre",
    description: "Comando de ejemplo",
    api_version: API_VERSION,
};

pub fn init(api: &mut Api) {
    api.command("hola", "saluda: hola [nombre]", hola);
}

fn hola(out: &mut dyn Write, args: &str) {
    let quien = if args.is_empty() { "mundo" } else { args };
    let _ = writeln!(out, "Hola, {}!", quien);
}
```

Todo plugin necesita dos cosas, con estos nombres exactos:

- `pub const INFO: PluginInfo` con sus datos.
- `pub fn init(api: &mut Api)`, que se llama una vez al arrancar.

### Ejemplos completos

Mira los de la rama Plugins:

| Para aprender... | Mira |
|---|---|
| lo minimo | `hola` |
| colores de acento | `temas_extra` |
| widget en la barra + comando | `sysmon` |
| app con teclado y un evaluador | `calculadora` |
| app con cursor y varias lineas | `notas` |
| app que se actualiza sola (`tick`) | `cronometro`, `viborita` |

### Depurar

- `api.log("texto")` o `crate::log!("x = {}", x)` escriben en el log del
  kernel: salen en la terminal donde ejecutaste `cargo run`.
- Un `panic!` muestra la pantalla de error de FR-OS con el mensaje.

---

## 7. Referencia de la API

Todo se importa con una linea:

```rust
use crate::plugins::api::*;
```

**Version actual de la API: 1** (`API_VERSION`).

### Lo que registras en `init(api)`

| Metodo | Para que sirve |
|---|---|
| `api.command(nombre, ayuda, funcion)` | Comando de la terminal. `funcion: fn(&mut dyn Write, &str)` recibe donde escribir y los argumentos. |
| `api.app(nombre, icono, fabrica)` | App en el lanzador. `icono: Option<fn(&mut Canvas, Rect, Color, Color)>` (con `None` usa la inicial). `fabrica: fn() -> Box<dyn App>`. |
| `api.widget(funcion)` | Texto en la barra superior. `funcion: fn() -> String`, se recalcula cada segundo. |
| `api.accent(nombre, color)` | Color de acento nuevo: `api.accent("rosa2", rgb(255, 80, 160))`. |
| `api.on_second(funcion)` | Llama a `fn()` una vez por segundo. |
| `api.log(texto)` | Escribe en el log del kernel. |

### Una app: el trait `App`

```rust
struct MiApp { /* tu estado */ }

impl App for MiApp {
    fn title(&self) -> &str { "MI APP" }
    fn content_size(&self) -> (i32, i32) { (400, 300) }     // ancho, alto
    fn draw(&mut self, canvas: &mut Canvas, area: Rect, focused: bool) { /* dibujar */ }
    fn on_key(&mut self, key: Key) -> bool { /* true = hay que redibujar */ false }
    fn tick(&mut self, ticks: u64) -> bool { /* animaciones; true = redibujar */ false }
}
```

`on_key` y `tick` son opcionales. `tick` se llama muchas veces por segundo:
devuelve `true` **solo** cuando algo cambio, para no gastar CPU.

Teclas (`Key`): `Char(c)`, `Enter`, `Backspace`, `Tab`, `Escape`, `Up`, `Down`,
`Left`, `Right`, `Delete`, `Home`, `End`.

### Dibujar (`Canvas`)

| Funcion | Que hace |
|---|---|
| `fill_rect(rect, color)` | rectangulo solido |
| `round_rect(rect, radio, color, alpha)` | rectangulo redondeado (alpha 255 = solido) |
| `shade_rect(rect, color, alpha)` | rectangulo translucido |
| `draw_text(texto, x, y, color)` | texto; devuelve su ancho |
| `draw_text_centered(texto, rect, color)` | texto centrado |
| `draw_text_scaled(texto, x, y, escala, color, alpha)` | texto grande |
| `text_width(texto)` | ancho en pixeles |
| `line(x0, y0, x1, y1, color, alpha)` | linea |
| `circle(cx, cy, r, color, alpha)` | circunferencia |
| `put(x, y, color)` | un pixel |

Las coordenadas son de pantalla; `draw` recibe el `area` donde puedes dibujar.
Los colores se crean con `rgb(r, g, b)`.

### Colores del tema (`theme::`)

`accent()`, `on_accent()`, `TEXT`, `MUTED`, `TERM_BG`, `TERM_FG`, `WINDOW_BG`,
`PANEL_BG`, `TILE_BG`, `OUTLINE`, `SEPARATOR`, `BLACK`, `WHITE`, `OK`,
`AMBER`, `ERROR`, y `RADIUS` (radio de las esquinas). Usarlos hace que tu
plugin cambie con el color de acento elegido.

### Utilidades

| Funcion | Devuelve |
|---|---|
| `ticks()` | ticks desde el arranque (`TICKS_PER_SECOND` por segundo) |
| `uptime_seconds()` | segundos desde el arranque |
| `heap_usage()` | `(bytes usados, bytes totales)` de memoria |
| `local_time()` | `(hora, minuto)` |

---

## 8. Publicar en la rama Plugins

### Estructura

Cada plugin es **una carpeta** dentro de `plugins/`:

```
plugins/
└── mi_plugin/
    ├── plugin.rs      <- obligatorio
    └── README.md      <- recomendado: que hace y como se usa
```

Reglas:

- Nombre de la carpeta en minusculas, con letras, numeros y `_`.
- `INFO` completo: nombre, version, autor y descripcion.
- `api_version: API_VERSION`.
- Sin dependencias nuevas: solo lo que exporta `crate::plugins::api`.
- Idealmente una licencia compatible (MIT) indicada en el README.

### Para proponer un plugin (Pull Request a la rama Plugins)

1. Haz fork del repositorio y crea tu rama desde `Plugins`.
2. Agrega tu carpeta en `plugins/`.
3. Comprueba: `cargo run` compila **sin errores ni avisos nuevos**, el plugin
   aparece en `plugins` y hace lo que dice.
4. Agrega una fila en `plugins/CATALOGO.md`.
5. Abre el Pull Request describiendo que hace y que pide permiso (por ejemplo
   si usa `unsafe` o toca hardware).

Lista de revision para quien acepta: codigo legible, sin `unsafe` innecesario,
sin bucles infinitos, `INFO` correcto, probado.

### Para quien mantiene el repositorio: crear la rama Plugins

Con el paquete `fr_os_plugins_v0.8.0.tar.gz` en la raiz del repositorio:

```bash
git checkout -b Plugins
tar xzf fr_os_plugins_v0.8.0.tar.gz      # crea/actualiza plugins/...
git add plugins
git commit -m "Plugins opcionales: calculadora, notas, cronometro, viborita, sysmon, temas, hola"
git push -u origin Plugins
```

La rama `main` queda **sin** plugins instalados (solo la plantilla y el
README), y cada persona elige cuales copiar.

---

## 9. Limites actuales

- **Hay que volver a ejecutar `cargo run`** para aplicar cambios. FR-OS
  todavia no tiene disco, sistema de archivos ni un cargador de programas,
  asi que no puede cargar un plugin nuevo mientras esta encendido. Es la
  limitacion de fondo; cuando existan, se podra instalar en caliente.
- **No se guardan datos.** Lo que un plugin escribe en memoria (por ejemplo
  las notas) se pierde al apagar, por la misma razon.
- **Sin aislamiento**: ver la seccion de seguridad.
- El lanzador muestra hasta 15 aplicaciones a la vez; si hay mas, se llega
  a ellas escribiendo en el buscador.
- Dos plugins no pueden definir el mismo comando: gana el primero (orden
  alfabetico por nombre de carpeta). Los comandos del sistema tienen
  prioridad sobre los de plugins.
- La API puede cambiar entre versiones mayores; por eso existe `api_version`.
