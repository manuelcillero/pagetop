# 🔃 Dependencias

`PageTop` está basado en [Rust](https://www.rust-lang.org/) y crece a hombros de gigantes
aprovechando algunas de las librerías (*crates*) más robustas y populares del
[ecosistema Rust](https://lib.rs), incluyendo:

  * [Actix Web](https://actix.rs/) para los servicios web.
  * [Tracing](https://github.com/tokio-rs/tracing) para la gestión de trazas y registro de eventos
    de la aplicación.
  * [Fluent templates](https://github.com/XAMPPRocky/fluent-templates), que integra
    [Fluent](https://projectfluent.org/) para internacionalizar las aplicaciones.
  * Además de otros crates adicionales que puedes explorar en los archivos `Cargo.toml` de `PageTop`
    y sus paquetes.


# ⌨️ Código

`PageTop` incorpora código de [config-rs](https://crates.io/crates/config) (versión
[0.11.0](https://github.com/mehcode/config-rs/tree/0.11.0)) de
[Ryan Leckey](https://crates.io/users/mehcode), por sus ventajas para leer y asignar a tipos seguros
las opciones de configuración, delegando la asignación a cada paquete, tema o aplicación.


# 🗚 FIGfonts

`PageTop` usa el paquete [figlet-rs](https://crates.io/crates/figlet-rs) desarrollado por
*yuanbohan* para mostrar un banner de presentación en el terminal con el nombre de la aplicación en
caracteres [FIGlet](http://www.figlet.org). Las fuentes incluidas en `pagetop/src/app` son:

  * [slant.flf](http://www.figlet.org/fontdb_example.cgi?font=slant.flf) de *Glenn Chappell*
  * [small.flf](http://www.figlet.org/fontdb_example.cgi?font=small.flf) de *Glenn Chappell*
    (predeterminada)
  * [speed.flf](http://www.figlet.org/fontdb_example.cgi?font=speed.flf) de *Claude Martins*
  * [starwars.flf](http://www.figlet.org/fontdb_example.cgi?font=starwars.flf) de *Ryan Youck*


# 📰 Plantillas

La página de inicio predeterminada está inspirada en este práctico
[tutorial](https://www.codewithfaraz.com/content/109/creating-a-unique-neobrutalism-portfolio-page-with-html-css-and-javascript)
realizado por [Faraz](https://www.codewithfaraz.com/) que crea una página de demostración en estilo
*Neobrutalismo*.


# 🎨 Icono

"La Criatura" sonriente es una simpática creación de [Webalys](https://www.iconfinder.com/webalys).
Forma parte de su colección [Nasty Icons](https://www.iconfinder.com/iconsets/nasty), disponible en
[ICONFINDER](https://www.iconfinder.com).
