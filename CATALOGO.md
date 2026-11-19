# Catalogo de plugins de FR-OS

Plugins opcionales. Copia SOLO las carpetas que quieras dentro de `plugins/`
de tu FR-OS y ejecuta `cargo run`. Guia: `docs/PLUGINS.md`.

| Plugin | Que agrega | Tipo |
|---|---|---|
| `hola` | comando `hola [nombre]` (ejemplo minimo) | comando |
| `temas_extra` | 6 colores de acento: plata, oliva, ocre, cobre, sangre, hielo | colores |
| `sysmon` | widget `MEM %` en la barra superior y comando `sysmon` | widget + comando |
| `calculadora` | app de calculadora en el lanzador y comando `calc` | app + comando |
| `notas` | bloc de notas (no se guarda al reiniciar) | app |
| `cronometro` | cronometro con vueltas | app |
| `viborita` | juego de la viborita | app |

Todos usan la API 1. Son codigo Rust que corre dentro del kernel: revisalos
antes de instalarlos.
