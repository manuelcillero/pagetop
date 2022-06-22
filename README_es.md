**PageTop** es un entorno de desarrollo basado en [Rust](https://www.rust-lang.org/es/) que reúne
algunos de los crates más estables y populares para crear soluciones web modulares, extensibles y
configurables.

Incluye **Drust**, un sistema de gestión de contenidos basado en PageTop que permite crear, editar y
mantener sitios web dinámicos, rápidos y seguros.


# Advertencia

**PageTop** es un proyecto personal para aprender Rust y conocer su ecosistema. Ahora mismo sólo se
liberan versiones de desarrollo. En este contexto la API no tiene ninguna estabilidad y los cambios
son constantes. No puede considerarse listo para probar hasta que se libere la versión **0.1.0**.


# Estructura del código

El repositorio se organiza en un *workspace* con los siguientes subproyectos:

* [pagetop](pagetop/), es la librería esencial construida con *crates* estables y muy conocidos del
  ecosistema Rust para proporcionar APIs, patrones de desarrollo y buenas prácticas para la creación
  avanzada de soluciones web SSR (*Server-Side Rendering*).

* [pagetop-admin](pagetop_admin/), módulo que proporciona a otros módulos un lugar común donde
  presentar a los administradores sus opciones de configuración.

* [pagetop-user](pagetop_user/), módulo para añadir una gestión de usuarios, roles, permisos y
  sesiones en aplicaciones desarrolladas con PageTop.

* [pagetop-node](pagetop_node/), módulo para crear y extender los tipos de contenido que puede
  gestionar una solución web de propósito general o personalizado.

* [drust](drust/), es una aplicación humildemente inspirada en [Drupal](https://www.drupal.org) que
  proporciona un CMS (*Content Management System*) o sistema de gestión de contenidos para construir
  sitios web administrados y configurables.


# Licencia

Este proyecto tiene licencia, de hecho se puede aplicar cualquiera de las siguientes a tu elección:

* Licencia Apache versión 2.0
  ([LICENSE-APACHE](https://gitlab.com/manuelcillero/pagetop/-/blob/main/LICENSE-APACHE) o
  [http://www.apache.org/licenses/LICENSE-2.0]).

* Licencia MIT
  ([LICENSE-MIT](https://gitlab.com/manuelcillero/pagetop/-/blob/main/LICENSE-MIT) o
  [http://opensource.org/licenses/MIT]).
