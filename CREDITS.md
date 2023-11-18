# ⌨️ Código

* PageTop incluye código de la versión [0.11.0](https://github.com/mehcode/config-rs/tree/0.11.0) de
  [config](https://crates.io/crates/config), de [Ryan Leckey](https://crates.io/users/mehcode), por
  las facilidades que ofrece frente a sus versiones más modernas para leer los ajustes de
  configuración y delegar su asignación a tipos seguros según los requerimientos de cada módulo,
  tema o aplicación.

* PageTop incorpora una versión adaptada del excelente *crate* [maud](https://crates.io/crates/maud)
  de [Chris Wong](https://crates.io/users/lambda-fairy) (versión
  [0.25.0](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud)), para añadir sus funcionalidades
  sin requerir la referencia a `maud` en el archivo `Cargo.toml` de cada proyecto.

* PageTop usa los reconocidos *crates* [SQLx](https://github.com/launchbadge/sqlx) y
  [SeaQuery](https://github.com/SeaQL/sea-query) para interactuar con bases de datos. Sin embargo,
  para restringir las migraciones a módulos, se ha integrado en el código una versión modificada de
  [SeaORM Migration](https://github.com/SeaQL/sea-orm/tree/master/sea-orm-migration) (versión
  [0.11.3](https://github.com/SeaQL/sea-orm/tree/0.11.3/sea-orm-migration/src)).


# 🗚 FIGfonts

PageTop utiliza el paquete [figlet-rs](https://crates.io/crates/figlet-rs) de *yuanbohan* para
mostrar en el terminal un rótulo de presentación con el nombre de la aplicación usando caracteres
[FIGlet](http://www.figlet.org). Las fuentes incluidas en `src/app` son:

* [slant.flf](http://www.figlet.org/fontdb_example.cgi?font=slant.flf) por *Glenn Chappell*.
* [small.flf](http://www.figlet.org/fontdb_example.cgi?font=small.flf) por *Glenn Chappell*
  (predeterminado).
* [speed.flf](http://www.figlet.org/fontdb_example.cgi?font=speed.flf) por *Claude Martins*.
* [starwars.flf](http://www.figlet.org/fontdb_example.cgi?font=starwars.flf) por *Ryan Youck*.


# 📰 Plantillas

* El diseño de la página predeterminada de inicio está basado en la plantilla
  [Zinc](https://themewagon.com/themes/free-bootstrap-5-html5-business-website-template-zinc) creada
  por [inovatik](https://inovatik.com/) y distribuida por [ThemeWagon](https://themewagon.com).


# 🎨 Icono

"La criatura" sonriente es una divertida creación de [Webalys](https://www.iconfinder.com/webalys).
Puede encontrarse en su colección [Nasty Icons](https://www.iconfinder.com/iconsets/nasty)
disponible en [ICONFINDER](https://www.iconfinder.com).