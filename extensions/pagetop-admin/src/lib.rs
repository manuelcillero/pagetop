/*!
<div align="center">

<h1>PageTop Admin</h1>

<p>Panel de administración extensible para <strong>PageTop</strong>.</p>

</div>

## Guía rápida

Declara la dependencia en tu `Cargo.toml` activando el motor de base de datos:

```toml
[dependencies]
pagetop-admin = { version = "...", features = ["sqlite"] }
```

Añade `&pagetop_admin::Admin` a las dependencias de tu extensión y declara tus
secciones, páginas o configuración:

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
        permission:  Some(&MyPermission::Config),
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
            Self::Config => "myapp.config".into(),
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

Para formularios de configuración enteramente automáticos (GET y POST gestionados
por `pagetop-admin`), usa `AdminPageKind::ConfigForm(schema)` en lugar de `View` y
no registres la ruta tú mismo.
*/

use pagetop::prelude::*;
use pagetop_seaorm::install_migrations;

include_locales!(LOCALES_ADMIN);

/// Ruta raíz del panel de administración. No es configurable: otras piezas del ecosistema (p. ej.
/// las rutas de `pagetop-user`, `/admin/user/...`) ya asumen este valor de forma literal.
pub(crate) const ADMIN_BASE_PATH: &str = "/admin";

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

    async fn initialize(&self) {
        install_migrations!(m20260629_000001_create_settings);
        registry::build();

        // Región neutra del core: cualquier tema puede decidir renderizarla (o no) sin que
        // `pagetop-admin` dependa de ninguno en concreto -- ver `CoreRegions::Aside`.
        InRegion::Global(&CoreRegions::Aside).add(component::AdminMenu::new());
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
