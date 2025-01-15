# Introducción

Si quieres aprender a construir soluciones web que rescaten la esencia de los orígenes usando HTML, CSS y JavaScript para crear páginas web ofrecidas desde un servidor, pero con la potencia de un lenguaje de programación rápido y seguro como Rust, entonces... ¡has llegado a buen puerto!


# ¿Qué es PageTop?

**PageTop** es un marco de desarrollo web que proporciona herramientas y patrones de diseño predefinidos para el desarrollo de soluciones web seguras, modulares y personalizables con *Renderizado desde el Servidor* ([SSR](#ssr)).

PageTop está desarrollado en el [lenguaje de programación Rust](https://www.rust-lang.org/) y se apoya sobre los hombros de auténticos gigantes, porque utiliza algunas de las librerías más estables y reconocidas del [ecosistema Rust](https://lib.rs) como:

  - [Actix Web](https://github.com/actix/actix-web), para la gestión de los servicios y del servidor web.
  - [Tracing](https://github.com/tokio-rs/tracing), para el sistema de diagnóstico y mensajes de registro estructurados.
  - [Fluent templates](https://github.com/XAMPPRocky/fluent-templates), que incorpora [Fluent](https://projectfluent.org/) para internacionalizar los proyectos.
  - [SeaORM](https://github.com/SeaQL/sea-orm), que usa [SQLx](https://github.com/launchbadge/sqlx) para modelar el acceso a bases de datos.
  - Integra versiones *ad hoc* de [config-rs](https://github.com/mehcode/config-rs) y [Maud](https://github.com/lambda-fairy/maud) en su código.
  - Y usa otras librerías que puedes ver en el archivo [`Cargo.toml`](https://github.com/manuelcillero/pagetop/blob/latest/Cargo.toml) de PageTop.


# SSR

El *Renderizado desde el Servidor* (SSR) es una técnica de desarrollo web en la que el contenido HTML se genera en el servidor antes de enviarlo al navegador del usuario, donde CSS y JavaScript añaden la interactividad necesaria. PageTop encapsula todos estos elementos en **componentes** unitarios que pueden mantenerse de forma independiente y ser extendidos o modificados por otras librerías.

Esto contrasta con la *Renderización desde el Cliente* (CSR), donde es el navegador el que genera el contenido HTML tras recibir el código WebAssembly o JavaScript necesario desde el servidor.

PageTop usa SSR como una solución robusta para la creación de soluciones web complejas. Pero también presenta desafíos, como ciclos de desarrollo más lentos por la necesidad de recompilar cada cambio en el código Rust. No obstante, ofrece excelentes tiempos de carga iniciales, mejora en el SEO, y unifica el desarrollo en cliente y servidor bajo un mismo lenguaje.


# Contribuciones

PageTop [empezó como un proyecto personal](https://manuel.cillero.es/blog/aprendiendo-rust-presentando-pagetop/) para aprender a programar con Rust. Es [libre y de código abierto](https://github.com/manuelcillero/pagetop#-license), para siempre. Y puedes contribuir aumentando su versatilidad, documentando, traduciendo o corrigiendo errores. Pero también puedes crear tus propias extensiones o temas que otros desarrolladores podrán utilizar en sus proyectos.


# Advertencia

PageTop está aún en las primeras etapas de desarrollo. Faltan características importantes y otras no funcionan como deberían. Y la documentación es escasa. Sólo se liberan versiones de desarrollo con cambios importantes en la API que desaconseja su uso en producción. Úsalo si estás interesado en conocerlo o quieres contribuir.

Si necesitas un entorno *fullstack* estable y robusto para tu próximo proyecto, puedes mirar [Perseus](https://github.com/framesurge/perseus) basado en la excelente librería [Sycamore](https://github.com/sycamore-rs/sycamore), también te entusiasmará [Rocket](https://github.com/rwf2/Rocket), sin descartar [MoonZoon](https://github.com/MoonZoon/MoonZoon) o [Percy](https://github.com/chinedufn/percy). Y puedes crear tu propio *framework* combinando soluciones como [Yew](https://yew.rs/), [Leptos](https://leptos.dev/) o [Dioxus](https://dioxuslabs.com/) con el servidor [Axum](https://github.com/tokio-rs/axum) y el ORM [Diesel](https://github.com/diesel-rs/diesel) para construir increíbles aplicaciones [SSR](https://en.wikipedia.org/wiki/Server-side_scripting).

Si aún sigues por aquí, ¡ha llegado el momento de empezar a aprender algo de PageTop!

La guía de [Inicio Rápido](getting-started.html) te enseñará a probar los ejemplos. También te ayudará con la [configuración](configuration.html) de tu entorno de desarrollo y te orientará con los próximos pasos a seguir.
