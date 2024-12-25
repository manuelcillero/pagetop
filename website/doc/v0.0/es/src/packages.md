# Paquetes

Una de las características más poderosas de PageTop es su extensibilidad mediante el uso de [Paquetes](https://docs.rs/pagetop/latest/pagetop/core/package/index.html). Los paquetes añaden, amplían o personalizan funcionalidades para nuestra aplicación.

Un paquete es una [estructura unitaria](https://stackoverflow.com/questions/67689613/what-is-a-real-world-example-of-using-a-unit-struct) (*unit struct*) que implementa el *trait* [`PackageTrait`](https://docs.rs/pagetop/latest/pagetop/core/package/trait.PackageTrait.html). Los métodos de [`PackageTrait`](https://docs.rs/pagetop/latest/pagetop/core/package/trait.PackageTrait.html) tienen un funcionamiento predefinido que se puede personalizar.

Los paquetes tienen acceso a puntos de nuestra aplicación donde PageTop permite que el código de terceros haga ciertas cosas.

## ¡Hola mundo!

Para añadir lógica a nuestra [aplicación](apps.html) puedes crear un paquete en tu archivo `main.rs` sustituyendo el código de ejemplo por este nuevo código:

```rust
use pagetop::prelude::*;

struct HelloWorld;

impl PackageTrait for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
```

La función `main()` instancia la aplicación usando el método `prepare()` con una referencia ([`PackageRef`](https://docs.rs/pagetop/latest/pagetop/core/package/type.PackageRef.html)) al paquete `HelloWorld`. Así se indica a PageTop que debe incluirlo en su registro interno de paquetes.

`HelloWorld` configura un servicio en la ruta de inicio ("/") que se implementa en `hello_world()`. Esta función devuelve una página web con un componente que directamente renderiza código HTML para mostrar un título con el texto *Hello World!*.

Ahora si compilamos y ejecutamos nuestra aplicación con `cargo run` y en el navegador volvemos a cargar la dirección `http://localhost:8088`, veremos el saludo esperado.

## Librerías

Los paquetes en PageTop son *crates* de biblioteca, usualmente publicados en [crates.io](https://crates.io/search?q=pagetop), que puedes usar como dependencias en tu aplicación.


# Seguridad

Los paquetes ajenos a PageTop contienen código desarrollado por terceros y, dado que pueden hacer básicamente lo que quieran, pueden representar un serio riesgo para la seguridad de tu sistema. Por ejemplo, un paquete podría indicar que está analizando la entrada del usuario y realmente está descargando ransomware en tu computadora.

Cualquier sospecha sobre paquetes malintencionados debe ser reportado confidencialmente al administrador de PageTop para ser analizado por la comunidad.
<!--
## El registro de paquetes

En este sitio web, se mantiene un registro de todos los paquetes conocidos. El ecosistema es joven. Los paquetes respaldados por la comunidad de PageTop tendrán una marca de verificación, aunque PageTop no se responsabiliza de ningún modo por paquetes malintencionados al ser código de terceros. Puedes añadir tus propios paquetes al registro siguiendo las instrucciones en nuestro sistema de reporte de issues, que te guiará a través del proceso.
-->
