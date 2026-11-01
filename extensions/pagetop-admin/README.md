<div align="center">

<h1>PageTop Admin</h1>

<p>Panel de administración extensible para <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-admin?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-admin)
[![Crates.io](https://img.shields.io/crates/v/pagetop-admin.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-admin)
[![Descargas](https://img.shields.io/crates/d/pagetop-admin.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-admin)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-admin#licencia)

</div>

Añade a una aplicación PageTop un panel de administración en `/admin` que cada extensión amplía con
sus propias secciones, páginas, pestañas y botones, declarados con acciones. Trae las secciones
habituales (personas, estructura, configuración, informes y ayuda), formularios de configuración
que se guardan en la base de datos y un menú de navegación hecho sólo con componentes del núcleo,
para que funcione con cualquier tema.

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-admin = "..."
```

La configuración se guarda con `pagetop-seaorm`: la aplicación elige el motor de base de datos con
su *feature* (`postgres`, `mysql` o `sqlite`) y configura la conexión en la sección `[database]`.

**Declara la extensión** como dependencia de la tuya (`&pagetop_admin::Admin`) y **añade al panel**
tus secciones, páginas o formularios de configuración:

```rust,no_run
use pagetop::prelude::*;
use pagetop_admin::prelude::*;
use pagetop_admin::settings::{SettingField, SettingsSchema};

pub struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_admin::Admin]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            DeclareAdminPages::new(declare_pages),
        ]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/admin/config/myapp", web::get(config_handler))
    }
}

fn declare_pages(bag: &mut PageBag) {
    bag.add(AdminPage {
        path:        "/admin/config/myapp".to_owned(),
        section:     "config".to_owned(),
        title:       Lc::n("My App"),
        description: Some(Lc::n("Configure My App.")),
        weight:      0,
        permission:  &MyPermission::Config,
        kind:        AdminPageKind::View,
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

async fn config_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let schema = SettingsSchema::new("myapp.config")
        .with_field(SettingField::text("site_name", "Site name").with_required(true));

    let title   = "My App configuration";
    let mut form = ConfigForm::with_schema(schema);
    let mut cx   = Context::new(request.clone());
    let content  = form.render(&mut cx).await;

    Page::admin(request)
        .with_title(Lc::n(title))
        .with_child(
            AdminFrame::new()
                .with_title(Lc::n(title))
                .with_child(Html::with(move |_| content.clone())),
        )
        .render()
        .await
}
```

Para formularios de configuración enteramente automáticos (GET y POST gestionados por
`pagetop-admin`), usa `AdminPageKind::ConfigForm(schema)` en lugar de `View` y no registres la ruta
tú mismo.

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
