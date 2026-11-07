<div align="center">

<h1>PageTop Admin</h1>

<p>Ajustes guardados en la base de datos para el panel de administración de <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-admin?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-admin)
[![Crates.io](https://img.shields.io/crates/v/pagetop-admin.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-admin)
[![Descargas](https://img.shields.io/crates/d/pagetop-admin.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-admin)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-admin#licencia)

</div>

Añade al panel de administración de PageTop **ajustes que se guardan en la base de datos** y que un
administrador cambia desde el propio panel, con la aplicación en marcha.

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Qué aporta

- **Páginas de ajustes sin código propio.** Una extensión describe los campos con un
  `SettingsSchema` y declara la página con `DeclareConfigForms`. `pagetop-admin` monta su ruta,
  pinta el formulario con los valores guardados y guarda lo que se envía, salvo que falte algún
  campo obligatorio o llegue vacío.
- **El almacén de los valores.** La tabla `settings`, que crea al arrancar, y las funciones del
  módulo `settings` para leerlos (`settings::get_or()` y compañía) y escribirlos desde código.
- **El formulario** (`ConfigForm`), por si una página propia quiere mostrar el de un esquema.

## En qué se apoya

El panel en sí es de PageTop (el módulo `pagetop::admin`): su página de inicio, sus secciones, la
navegación, las migas de pan y la plantilla de administración (`Page::admin()`). Cada página de
ajustes es una `AdminPage` del núcleo, así que aparece en el panel como cualquier otra, con su
sección y su permiso, y se pinta con cualquier tema. Una extensión que sólo añade páginas propias al
panel, sin ajustes, no necesita `pagetop-admin`.

## Cuándo usarlo

Para valores que decide quien administra el sitio y que cambian con el tiempo: un aviso en la
portada, un número de plazas, abrir o cerrar las inscripciones... Lo que fija quien despliega la
aplicación (la base de datos, el servidor de correo, los límites) va en los archivos de
configuración (ver el módulo `pagetop::config`), que no se pueden cambiar desde el panel y exigen
reiniciar.

Los valores que envía el formulario se guardan como texto, también los de los campos numéricos y las
casillas (`"42"`, `"true"`): se leen como `String` y se convierten al usarlos.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-admin = "..."
```

Los ajustes se guardan con `pagetop-seaorm`: la aplicación elige el motor de base de datos con su
*feature* (`postgres`, `mysql` o `sqlite`) y configura la conexión en la sección `[database]`.

**Declara la extensión** como dependencia de la tuya (`&pagetop_admin::Admin`); si no, sus páginas
de ajustes no se montan. Después **declara tus páginas de ajustes**, cada una con su permiso:

```rust,no_run
use pagetop::prelude::*;
use pagetop_admin::prelude::*;

pub struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_admin::Admin]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![DeclareConfigForms::new(declare_forms)]
    }
}

fn declare_forms(bag: &mut ConfigFormBag) {
    bag.add(ConfigFormPage {
        page: AdminPage::new("config/myapp", "config", Lc::n("My App"), &MyPermission::Config)
            .with_description(Lc::n("Name of the site.")),
        schema: SettingsSchema::new("myapp").with_field(
            SettingField::text("site_name", Lc::n("Site name")).with_required(true),
        ),
    });
}

#[derive(Clone, Copy, Debug)]
enum MyPermission {
    Config,
}

impl Permission for MyPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::Config => "myapp:config".into(),
        }
    }
}

// Los valores se guardan con la clave `<ámbito>.<campo>`.
async fn site_name() -> String {
    pagetop_admin::settings::get_or("myapp.site_name", String::new()).await
}
```

## Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.

## Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
