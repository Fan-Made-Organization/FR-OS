# Política de seguridad, copyright y aviso legal

## Autores y contacto

| Autor | Contacto (Discord) |
| ----- | ------------------ |
| Sam | `sam_99709` |
| X504 | `x504_produce` |

Para cualquier consulta sobre el proyecto, permisos de uso, créditos o
reportes, escríbeles por Discord a 'X504'.

---

## 1. Versiones soportadas

FR-OS es un proyecto personal y experimental. Solo la versión más reciente
recibe correcciones.

| Versión | Soportada          |
| ------- | ------------------ |
| 0.4.x  | :white_check_mark: |
| < 0.4   | :x:                |

> **Aviso importante:** este es un sistema operativo en desarrollo. Puede
> fallar, congelarse o perder datos. Úsalo en una máquina virtual (QEMU) o en
> un equipo de pruebas, **no** en un equipo con información importante.

---

## 2. Cómo reportar una vulnerabilidad o un error grave

**No publiques vulnerabilidades en un issue abierto**, porque cualquiera las
vería antes de que estén corregidas.

1. Usa la pestaña **Security** de este repositorio y elige
   **Report a vulnerability** (reporte privado), o
2. escribe por mensaje privado en Discord a `x504_produce`.

Incluye siempre:
- La versión que usas (por ejemplo, 4.0.1).
- Si lo probaste en QEMU o en hardware real.
- Pasos para reproducir el problema.
- Qué pasa y qué esperabas que pasara.

Es un proyecto personal, así que se responde con el mejor esfuerzo posible y
**no se garantizan plazos**. Si el reporte es válido, se corregirá y se dará
crédito a quien lo reportó, si así lo desea.

Los errores comunes (algo que no funciona, una tecla que no coincide, un
cuelgue) sí pueden ir como issue normal.

---

## 3. Copyright

Copyright (c) 2026 sam_99709 y x504_produce.

El código fuente, el diseño de la interfaz, los textos y la documentación de
FR-OS son obra de sus autores y están protegidos por derechos de autor. Eso
significa que **la autoría es de ellos y se debe respetar**, aunque el
software sea gratuito.

### ¿Qué significa tener copyright y a la vez ser gratis?

Son dos cosas distintas y no se contradicen:

- **Copyright** = quién es el autor y qué se puede hacer con la obra.
- **Gratis** = que no se cobra por usarlo.

Los autores conservan el copyright, pero **otorgan permiso gratuito** para
usar el software según la licencia del archivo `LICENSE`.

---

## 4. Uso gratuito: qué puedes y qué no

**Este software es y seguirá siendo gratis.**

Según la licencia del proyecto, puedes, sin pagar:

- Descargarlo y ejecutarlo.
- Estudiar su código y aprender de él.
- Modificarlo para tu propio uso.
- Compartir copias y versiones modificadas.

A cambio, debes:

- **Conservar el aviso de copyright y la licencia** en cualquier copia o
  versión derivada.
- **Dar crédito** a los autores originales.

No está permitido:

- Quitar o ocultar el nombre de los autores.
- Presentar el proyecto como si fuera tu obra original.
- Usar los nombres de los autores para promocionar tu versión como si ellos
  la respaldaran.

> Si los términos exactos de uso te generan dudas, **lo que manda es el
> archivo `LICENSE`**. Este documento es una explicación en lenguaje simple.

---

## 5. Sin garantía

El software se entrega **"tal cual", sin garantía de ningún tipo**, ni
explícita ni implícita, incluyendo (sin limitarse a) garantías de que
funcione, de que sirva para un fin concreto o de que no tenga errores.

Los autores **no son responsables** de daños, pérdida de datos, fallos de
hardware u otros perjuicios derivados del uso del software. Lo usas bajo tu
propio riesgo.

---

## 6. Componentes y fuentes de terceros

FR-OS usa bibliotecas y recursos de terceros, por ejemplo:

- Crates de Rust (`bootloader`, `x86_64`, `linked_list_allocator`,
  `lazy_static`, entre otros).
- La fuente bitmap Noto Sans Mono (`noto-sans-mono-bitmap`).

Cada uno conserva **su propia licencia y a sus propios autores**, y esa
licencia aplica a ese componente. El copyright de este proyecto no cubre ni
reemplaza el de ellos.

---

## 7. Sin afiliación

FR-OS es un proyecto independiente.

- El estilo visual está **inspirado** en una estética musical de manera
  personal. Cualquier mención a artistas, álbumes, canciones o marcas es solo
  referencia o inspiración.
- Este proyecto **no está afiliado, patrocinado ni respaldado** por ninguno de
  esos artistas, sellos o entidades.
- Sus nombres, marcas, imágenes y obras **pertenecen a sus respectivos
  dueños**. FR-OS no incluye ni redistribuye portadas, logos, música ni
  tipografías de ellos.
- Es un proyecto en **Honor** a Fanta Rosario, por todo lo que ha hecho este 2026.

---

## 8. Privacidad

FR-OS no recopila, guarda ni envía datos personales. Todo se ejecuta
localmente en tu máquina.

---

## 9. Cambios a este documento

Este documento puede actualizarse con el tiempo. La versión vigente es la que
está en la rama principal del repositorio.
