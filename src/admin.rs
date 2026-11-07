//! Panel de administración: secciones, páginas, pestañas y botones para las extensiones.
//!
//! Cada extensión aporta sus piezas con las acciones de [`action::admin`]
//! ([`DeclareAdminSections`], [`DeclareAdminPages`], [`DeclareAdminTasks`] y
//! [`DeclareAdminActions`]), sin depender de ninguna otra extensión. PageTop las reúne al arrancar
//! en un registro ([`AdminRegistry`]), que no cambia después.
//!
//! Todas las rutas se declaran **relativas** a la raíz del panel, el ajuste
//! [`admin_path`](crate::global::AppSettings::admin_path) (*"/admin"* por defecto):
//! `"config/site"` es la página `/admin/config/site`. [`admin_path()`] devuelve la ruta completa,
//! para enlazar; los *handlers* se montan en [`Extension::configure_router()`] con
//! [`Routes::admin_route()`], que también recibe la ruta relativa.
//!
//! Si hay alguna página declarada, PageTop monta en la raíz del panel su página de inicio
//! (*dashboard*), con todas las secciones y sus páginas, y en la ruta de cada sección con páginas,
//! una página con las suyas (salvo que una página declarada ocupe esa misma ruta). Las páginas de
//! administración se crean con [`Page::admin()`], cuya plantilla añade la navegación del panel,
//! las migas de pan, el título y las pestañas y botones declarados para cada ruta.
//!
//! Cada página declara un permiso ([`Permission`]), el único sitio donde se indica: decide si se
//! muestra en la navegación y protege sus rutas. Éstas se montan con [`Routes::admin_route()`], que
//! exige el permiso de la página de la que cuelgan (`"config/export/{id}/edit"` cuelga de
//! `"config/export"`) antes de llegar al *handler*; una ruta del panel montada de otra forma, o que
//! no cuelgue de ninguna página, impide arrancar la aplicación. Un permiso más concreto lo
//! comprueba el *handler*. Sin una extensión de autenticación que conceda los permisos (ver
//! [`CheckPermission`]), nadie puede acceder a ninguna.
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! #[derive(Clone, Copy, Debug)]
//! enum ToolsPermission {
//!     Export,
//! }
//!
//! impl Permission for ToolsPermission {
//!     fn key(&self) -> CowStr {
//!         match self {
//!             Self::Export => "tools:export".into(),
//!         }
//!     }
//! }
//!
//! fn declare_pages(bag: &mut AdminPageBag) {
//!     bag.add(
//!         AdminPage::new("config/export", "config", Lc::n("Export"), &ToolsPermission::Export)
//!             .with_description(Lc::n("Export site data.")),
//!     );
//! }
//!
//! // PageTop ya ha comprobado el permiso de la página antes de llegar aquí.
//! async fn export(request: HttpRequest) -> Result<Markup, ErrorPage> {
//!     Page::admin(request)
//!         .with_title(Lc::n("Export"))
//!         .render()
//!         .await
//! }
//!
//! pub struct Tools;
//!
//! #[async_trait]
//! impl Extension for Tools {
//!     fn actions(&self) -> Vec<ActionBox> {
//!         actions![action::admin::DeclareAdminPages::new(declare_pages)]
//!     }
//!
//!     fn configure_router(&self, routes: Routes) -> Routes {
//!         routes.admin_route("config/export", web::get(export))
//!     }
//! }
//! ```
//!
//! [`Extension::configure_router()`]: crate::core::extension::Extension::configure_router
//! [`Page::admin()`]: crate::response::Page::admin
//! [`Permission`]: crate::auth::Permission
//! [`CheckPermission`]: crate::base::action::auth::CheckPermission
//! [`Routes::admin_route()`]: crate::web::Routes::admin_route
//! [`action::admin`]: crate::base::action::admin
//! [`DeclareAdminSections`]: crate::base::action::admin::DeclareAdminSections
//! [`DeclareAdminPages`]: crate::base::action::admin::DeclareAdminPages
//! [`DeclareAdminTasks`]: crate::base::action::admin::DeclareAdminTasks
//! [`DeclareAdminActions`]: crate::base::action::admin::DeclareAdminActions

use crate::core::component::Context;
use crate::{global, util};

mod bag;
pub use bag::{AdminActionBag, AdminPageBag, AdminSectionBag, AdminTaskBag};

mod registry;
pub use registry::{AdminAction, AdminPage, AdminRegistry, AdminSection, AdminTask};
pub(crate) use registry::{build_admin_registry, undeclared_permission};

mod pages;
pub(crate) use pages::configure_routes;

/// Devuelve la ruta completa de una ruta relativa a la raíz del panel de administración.
///
/// Con la raíz por defecto, `admin_path("config/site")` devuelve `"/admin/config/site"` y
/// `admin_path("")`, la propia raíz (`"/admin"`). Se ignoran las `/` del principio y del final.
pub fn admin_path(path: &str) -> String {
    let root = &global::SETTINGS.app.admin_path;
    let path = path.trim_matches('/');
    if path.is_empty() {
        root.clone()
    } else {
        util::join!(root, "/", path)
    }
}

/// Devuelve la ruta relativa a la raíz del panel de administración de una ruta completa.
///
/// Devuelve `None` si no está dentro del panel. Es la inversa de [`admin_path()`]: con la raíz por
/// defecto, `"/admin/config/site"` devuelve `Some("config/site")`, `"/admin"` devuelve `Some("")` y
/// `"/administrator"`, `None`.
pub fn strip_admin_path(path: &str) -> Option<&str> {
    let rest = path.strip_prefix(global::SETTINGS.app.admin_path.as_str())?;
    if rest.is_empty() {
        Some(rest)
    } else {
        rest.strip_prefix('/')
            .map(|rest| rest.trim_end_matches('/'))
    }
}

/// Devuelve `true` si el usuario de la petición puede acceder a alguna página del panel.
///
/// Permite a una extensión (p. ej. un menú de cuenta) ofrecer el enlace al panel sólo cuando
/// procede.
pub fn can_access_admin(cx: &Context) -> bool {
    let registry = AdminRegistry::global();
    registry
        .pages()
        .iter()
        .any(|page| registry.section(page.section()).is_some() && page.is_accessible(cx))
}
