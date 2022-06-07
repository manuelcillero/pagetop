[PageTop](https://suitepro.cillero.es/projects/pagetop/wiki) es un proyecto basado en el [lenguaje
de programación Rust](https://www.rust-lang.org/es/) que reúne algunos de los crates más estables y
populares en un único entorno para crear soluciones web modulares, extensibles y configurables.
Incluye **Drust**, un sistema de gestión de contenidos para crear sitios web dinámicos, rápidos y
seguros.


# Advertencia

**PageTop** es un proyecto personal para aprender a programar con Rust y conocer su ecosistema.
Ahora mismo sólo se liberan versiones de desarrollo con pruebas de concepto. En este contexto la API
no tiene ninguna estabilidad y los cambios son constantes. Básicamente aún no hace nada. No puede
considerarse listo para usar en producción hasta que se libere la versión **0.1.0**.


# Estructura del código

El repositorio se organiza en un *workspace* con los siguientes proyectos (*crates*):

* [pagetop](pagetop/), es la librería esencial, reúne algunos de los crates más estables y
  populares del ecosistema Rust para proporcionar APIs, patrones de desarrollo y buenas prácticas
  para la creación avanzada de soluciones web.

* [pagetop-admin](pagetop_admin/), habilita a otros módulos un entorno central para la
  configuración de las aplicaciones.

* [pagetop-user](pagetop_user/), para la gestión de usuarios, roles, permisos y sesiones en
  aplicaciones desarrolladas con PageTop.

* [pagetop-node](pagetop_node/), proporciona un entorno básico para crear contenidos y extenderlos
  con funcionalidades básicas para, por ejemplo, crear un sitio básico con páginas, entradas de blog
  y comentarios.

* [drust](drust/), integra la librería y las extensiones anteriores de PageTop en un sistema de
  gestión de contenidos para crear sitios web dinámicos, rápidos y seguros.


# Licencia

Este proyecto tiene licencia, de hecho se puede aplicar cualquiera de las siguientes a tu elección:

* Licencia Apache versión 2.0
  ([LICENSE-APACHE](https://gitlab.com/manuelcillero/pagetop/-/blob/main/LICENSE-APACHE) o
  [http://www.apache.org/licenses/LICENSE-2.0]).

* Licencia MIT
  ([LICENSE-MIT](https://gitlab.com/manuelcillero/pagetop/-/blob/main/LICENSE-MIT) o
  [http://opensource.org/licenses/MIT]).
