# Extensiones

Una de las características más poderosas de PageTop es su extensibilidad mediante el uso de [Extensiones](https://docs.rs/pagetop/latest/pagetop/core/extension/index.html). Las extensiones añaden, amplían o personalizan funcionalidades para nuestra aplicación.

Una extensión es una [estructura unitaria](https://stackoverflow.com/questions/67689613/what-is-a-real-world-example-of-using-a-unit-struct) (*unit struct*) que implementa el *trait* [`ExtensionTrait`](https://docs.rs/pagetop/latest/pagetop/core/extension/trait.ExtensionTrait.html). Los métodos de [`ExtensionTrait`](https://docs.rs/pagetop/latest/pagetop/core/extension/trait.ExtensionTrait.html) tienen un funcionamiento predefinido que se puede personalizar.

Las extensiones tienen acceso a puntos de nuestra aplicación donde PageTop permite que el código de terceros haga ciertas cosas.

## ¡Hola mundo!

Para añadir lógica a nuestra [aplicación](apps.html) puedes crear una extensión en tu archivo `main.rs` sustituyendo el código de ejemplo por este nuevo código:

```rust
use pagetop::prelude::*;

struct HelloWorld;

impl ExtensionTrait for HelloWorld {
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

La función `main()` instancia la aplicación usando el método `prepare()` con una referencia ([`ExtensionRef`](https://docs.rs/pagetop/latest/pagetop/core/extension/type.ExtensionRef.html)) a la extensión `HelloWorld`. Así se indica a PageTop que debe incluirlo en su registro interno de extensiones.

`HelloWorld` configura un servicio en la ruta de inicio ("/") que se implementa en `hello_world()`. Esta función devuelve una página web con un componente que directamente renderiza código HTML para mostrar un título con el texto *Hello World!*.

Ahora si compilamos y ejecutamos nuestra aplicación con `cargo run` y en el navegador volvemos a cargar la dirección `http://localhost:8088`, veremos el saludo esperado.

## Librerías

Las extensiones en PageTop son *crates* de biblioteca, usualmente publicados en [crates.io](https://crates.io/search?q=pagetop), que puedes usar como dependencias en tu aplicación.


# Seguridad

Las extensiones ajenas a PageTop contienen código desarrollado por terceros y, dado que pueden hacer básicamente lo que quieran, pueden representar un serio riesgo para la seguridad de tu sistema. Por ejemplo, una extensión podría indicar que está analizando la entrada del usuario y realmente está descargando ransomware en tu computadora.

Cualquier sospecha sobre extensiones malintencionadas debe ser reportada confidencialmente al administrador de PageTop para ser analizado por la comunidad.
<!--
## El registro de extensiones

En este sitio web, se mantiene un registro de todas las extensiones conocidas. El ecosistema es joven. Las extensiones respaldadas por la comunidad de PageTop tendrán una marca de verificación, aunque PageTop no se responsabiliza de ningún modo por extensiones malintencionadas al ser código de terceros. Puedes añadir tus propias extensiones al registro siguiendo las instrucciones en nuestro sistema de reporte de issues, que te guiará a través del proceso.
-->
