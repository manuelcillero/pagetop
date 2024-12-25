# Aplicaciones

Los programas de PageTop se denominan [Aplicaciones](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html). La aplicación PageTop más simple luce así:

```rust
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```

La línea `use pagetop::prelude::*;` sirve para importar la API esencial de PageTop. Por brevedad, esta guía podría omitirla en ejemplos posteriores.

Ahora sólo tienes que copiar el código anterior en tu archivo `main.rs` y desde la carpeta del proyecto ejecutar:

```bash
cargo run
```

Si todo ha ido bien, después de compilar el código se ejecutará la aplicación. El terminal quedará en espera mostrando el *nombre* y *lema* predefinidos.

Ahora abre un navegador en el mismo equipo y escribe `http://localhost:8088` en la barra de direcciones. Y ya está, ¡la página de presentación de PageTop te dará la bienvenida!

Sin embargo, aún no hemos indicado a nuestra aplicación qué hacer.


# Extendiendo PageTop

La [API de PageTop](https://docs.rs/pagetop) ofrece cuatro instrumentos esenciales para construir una aplicación:

  - [**Paquetes**](packages.md), que añaden, amplían o personalizan funcionalidades interactuando con la API de PageTop o las APIs de paquetes de terceros.
  - [**Componentes**](components.md), para encapsular HTML, CSS y JavaScript en unidades funcionales, configurables y bien definidas.
  - [**Acciones**](actions.md), alteran el comportamiento interno de otros elementos de PageTop interceptando su flujo de ejecución.
  - [**Temas**](themes.md), son *paquetes* que permiten a los desarrolladores cambiar la apariencia de páginas y componentes sin afectar su funcionalidad.

Si quieres saber más sobre el funcionamiento interno de las aplicaciones, continúa leyendo. Si no, puedes saltar a la siguiente página y empezar a añadir lógica a nuestra primera aplicación.


# ¿Qué hace una aplicación?

Como hemos visto arriba, primero se instancia la [Aplicación](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html). Podemos hacerlo usando [`new()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.new), como en el ejemplo, o con [`prepare()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.prepare), que veremos en la siguiente página. Ambos se encargan de iniciar los diferentes subsistemas de PageTop por este orden:

  1. Inicializa la traza de mensajes de registro y eventos.

  2. Valida el identificador global de idioma.

  3. Conecta con la base de datos.

  4. Registra los paquetes de la aplicación según sus dependencias internas.

  5. Registra las acciones de los paquetes.

  6. Inicializa los paquetes.

  7. Ejecuta las actualizaciones pendientes de la base de datos.

Pero no ejecuta la aplicación. Para eso se usa el método [`run()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.run), que arranca el servidor web para empezar a responder las peticiones desde cualquier navegador.

Hablaremos más de todos estos subsistemas en próximas páginas. Mientras tanto, ¡vamos a añadir algo de lógica a nuestra aplicación creando un paquete con un nuevo servicio web!
