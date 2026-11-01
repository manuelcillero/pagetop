/*!
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
*/

use pagetop::prelude::*;
use pagetop_seaorm::install_migrations;

include_locales!(LOCALES_ADMIN);

/// Ruta raíz del panel de administración. No es configurable: otras piezas del ecosistema (p. ej.
/// las rutas de `pagetop-user`, `/admin/user/...`) ya asumen este valor de forma literal. Es
/// pública para que otras extensiones enlacen al panel sin repetir la ruta.
pub const ADMIN_BASE_PATH: &str = "/admin";

pub mod action;
pub mod component;
pub mod error;
pub mod registry;
pub mod settings;

pub(crate) mod entity;
pub(crate) mod handlers;
pub(crate) mod migration;
pub(crate) mod seed;

pub use action::{
    ActionBag, DeclareAdminActions, DeclareAdminPages, DeclareAdminSections, DeclareAdminTasks,
    PageBag, SectionBag, TaskBag,
};
pub use component::{AdminFrame, AdminMenu, ConfigForm};
pub use registry::{AdminAction, AdminPage, AdminPageKind, AdminSection, AdminTask};
pub use settings::{SettingField, SettingFieldType, SettingsSchema};

/// Prelude de `pagetop-admin`.
pub mod prelude {
    pub use crate::action::{
        ActionBag, DeclareAdminActions, DeclareAdminPages, DeclareAdminSections, DeclareAdminTasks,
        PageBag, SectionBag, TaskBag,
    };
    pub use crate::component::{AdminFrame, AdminMenu, ConfigForm};
    pub use crate::error::AdminError;
    pub use crate::registry::{AdminAction, AdminPage, AdminPageKind, AdminSection, AdminTask};
    pub use crate::settings::{SettingField, SettingFieldType, SettingsSchema};
}

// **< Extension >**********************************************************************************

/// Implementa la extensión `pagetop-admin`.
pub struct Admin;

#[async_trait]
impl Extension for Admin {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_ADMIN)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_ADMIN)
    }

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_seaorm::SeaORM]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![action::DeclareAdminSections::new(seed::declare_default_sections).with_weight(-99),]
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        install_migrations!(m20260629_000001_create_settings)?;
        registry::build();

        // Región neutra del core: cualquier tema puede decidir renderizarla (o no) sin que
        // `pagetop-admin` dependa de ninguno en concreto (ver `CoreRegions::Aside`).
        InRegion::Global(&CoreRegions::Aside).add(component::AdminMenu::new());
        Ok(())
    }

    fn configure_router(&self, router: Router) -> Router {
        let base = ADMIN_BASE_PATH;
        let reg = registry::global();

        let mut r = router.route(base, web::get(handlers::dashboard));

        // Página de aterrizaje de cada sección, salvo que una extensión ya haya reclamado esa
        // misma ruta con su propia `AdminPage` (esa página gana y se monta más abajo).
        for section in reg.sections().values() {
            if !reg.pages().contains_key(&section.path) {
                r = r.route(section.path.as_str(), web::get(handlers::section_page));
            }
        }

        for (path, page) in reg.pages() {
            if let AdminPageKind::ConfigForm(_) = &page.kind {
                r = r.route(
                    path.as_str(),
                    web::get(handlers::config_form_get).post(handlers::config_form_post),
                );
            }
        }

        r
    }
}
