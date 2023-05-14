<div align="center">

  <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/banner/pagetop.png" />

  <h1>PageTop</h1>

  [![crate](https://img.shields.io/crates/v/pagetop.svg)](https://crates.io/crates/pagetop)
  [![docs](https://docs.rs/pagetop/badge.svg)](https://docs.rs/pagetop)

</div>

**PageTop** es un entorno de desarrollo basado en [Rust](https://www.rust-lang.org/es/) que reúne
algunos de los crates más estables y populares para crear soluciones web modulares, extensibles y
configurables.

Incluye **Drust**, un sistema de gestión de contenidos basado en PageTop que permite crear, editar y
mantener sitios web dinámicos, rápidos y seguros.


# 🚧 Advertencia

**PageTop** es un proyecto personal para aprender Rust y conocer su ecosistema. Sólo se liberan
versiones de desarrollo. En este contexto la API no es estable y los cambios son constantes. No
puede considerarse preparado hasta que se libere la versión **0.1.0**.


# 📂 Estructura del código

El repositorio se organiza en un *workspace* con los siguientes subproyectos:

* **[pagetop](https://github.com/manuelcillero/pagetop/tree/main/pagetop)**, es la librería esencial
  construida con *crates* estables y muy conocidos del ecosistema Rust para proporcionar APIs,
  patrones de desarrollo y buenas prácticas para la creación avanzada de soluciones web SSR
  (*Server-Side Rendering*).

## Extensiones

* **[pagetop-macros](https://github.com/manuelcillero/pagetop/tree/main/pagetop-macros)**, agrupa
  las principales macros procedurales para usar desde **PageTop**.

* **[pagetop-build](https://github.com/manuelcillero/pagetop/tree/main/pagetop-build)**, permite
  incluir fácilmente recursos en los archivos binarios al compilar aplicaciones creadas con
  **PageTop**.

## Componentes

* **[pagetop-minimal](https://github.com/manuelcillero/pagetop/tree/main/pagetop-minimal)**, módulo
  que proporciona un conjunto básico de componentes para la composición de páginas.

* **[pagetop-jquery](https://github.com/manuelcillero/pagetop/tree/main/pagetop-jquery)**, módulo
  que permite añadir jQuery en las páginas que incluyen componentes o temas que usen esta librería
  JavaScript para interactuar con el documento HTML.​

* **[pagetop-megamenu](https://github.com/manuelcillero/pagetop/tree/main/pagetop-megamenu)**,
  módulo que proporciona un nuevo componente para incluir menús avanzados en las aplicaciones web
  creadas con **PageTop**.

## Temas

* **[pagetop-aliner](https://github.com/manuelcillero/pagetop/tree/main/pagetop-aliner)**, tema que
  delimita con cajas los elementos HTML para mostrar esquemáticamente la composición de las páginas.

* **[pagetop-bootsier](https://github.com/manuelcillero/pagetop/tree/main/pagetop-bootsier)**, tema
  que utiliza el *framework* [Bootstrap](https://getbootstrap.com/) para la composición de páginas y
  visualización de componentes.

* **[pagetop-bulmix](https://github.com/manuelcillero/pagetop/tree/main/pagetop-bulmix)**, tema que
  utiliza el *framework* [Bulma](https://bulma.io/) para la composición de páginas y visualización
  de componentes.

## Módulos

* **[pagetop-homedemo](https://github.com/manuelcillero/pagetop/tree/main/pagetop-homedemo)**,
  módulo que muestra una página de inicio de demostración para presentar **PageTop**.

* **[pagetop-admin](https://github.com/manuelcillero/pagetop/tree/main/pagetop-admin)**, módulo que
  proporciona a otros módulos un lugar común donde presentar a los administradores sus opciones de
  configuración.

* **[pagetop-user](https://github.com/manuelcillero/pagetop/tree/main/pagetop-user)**, módulo para
  añadir gestión de usuarios, roles, permisos y sesiones en aplicaciones desarrolladas con PageTop.

* **[pagetop-node](https://github.com/manuelcillero/pagetop/tree/main/pagetop-node)**, módulo para
  crear, extender o personalizar los tipos de contenido que puede administrar un sitio web.

## Aplicación

* **[drust](https://github.com/manuelcillero/pagetop/tree/main/drust)**, es una aplicación
  inspirada modestamente en [Drupal](https://www.drupal.org) que utiliza PageTop para crear un CMS
  (*Content Management System* o sistema de gestión de contenidos) para construir sitios web
  dinámicos, administrados y configurables.


# 📜 Licencia

Este proyecto tiene licencia, de hecho tiene dos, puedes aplicar cualquiera de las siguientes a tu
elección:

* Licencia Apache versión 2.0
  ([LICENSE-APACHE](https://github.com/manuelcillero/pagetop/blob/main/LICENSE-APACHE) o
  [http://www.apache.org/licenses/LICENSE-2.0]).

* Licencia MIT
  ([LICENSE-MIT](https://github.com/manuelcillero/pagetop/blob/main/LICENSE-MIT) o
  [http://opensource.org/licenses/MIT]).
