//! Definiciones y plantillas del tema Bootsier.
//!
//! El módulo [`bs`] expone todos los tipos y componentes disponibles. Para usarlos sin ambigüedad
//! junto a `use pagetop::prelude::*`, y cargar también los traits del tema, importa este módulo
//! con glob:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::*;
//! ```
//!
//! # Plantillas
//!
//! Bootsier maqueta las dos plantillas de PageTop
//! ([`CoreTemplates`](pagetop::prelude::CoreTemplates)): `Standard`, con
//! cabecera, contenido y pie, que es la plantilla por defecto de cualquier página; y `Admin`, con
//! la shell completa de AdminLTE 4 (barra superior + barra lateral + área de contenido), que se
//! activa creando la página con [`Page::admin()`](pagetop::response::Page::admin) en lugar de
//! [`Page::new()`](pagetop::response::Page::new). No define sus propias variantes de plantilla:
//! intercepta el componente `Template` en `handle_component()` (ver `bs::layout`).
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! async fn about(request: HttpRequest) -> Result<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_child(Html::with(|_| html! {
//!             h1 { "Sobre nosotros" }
//!             p { "Texto de presentación." }
//!         }))
//!         .render().await
//! }
//! ```
//!
//! # Barra lateral
//!
//! Registra elementos en [`BootsierRegions::Sidebar`](bs::BootsierRegions::Sidebar) para poblar la
//! barra lateral de la shell. Los elementos esperados son [`bs::sidebar::Item`] y
//! [`bs::sidebar::Section`].
//!
//! De forma **global** (visibles en todas las páginas de administración):
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::bs::{BootsierRegions, sidebar};
//!
//! fn register_navigation() {
//!     InRegion::Global(&BootsierRegions::Sidebar)
//!         .add(sidebar::Section::titled(Lc::n("Administración")))
//!         .add(sidebar::Item::link(Lc::n("Usuarios"), "/users", "people"))
//!         .add(sidebar::Item::link(Lc::n("Roles"), "/roles", "shield-check"));
//! }
//! ```
//!
//! O de forma **por página**:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::bs::{BootsierRegions, sidebar};
//!
//! async fn settings(request: HttpRequest) -> Result<Markup, ErrorPage> {
//!     Page::admin(request)
//!         .with_child_in(
//!             &BootsierRegions::Sidebar,
//!             sidebar::Item::link(Lc::n("Ajustes"), "/settings", "gear"),
//!         )
//!         .with_child(Html::with(|_| html! { h3 { "Ajustes" } }))
//!         .render().await
//! }
//! ```
//!
//! # Barra de navegación superior
//!
//! La barra superior incluye por defecto los controles de pantalla completa y selector de tema.
//! Para añadir elementos adicionales en el lado derecho (por ejemplo, el dropdown de usuario de
//! `pagetop-user`), registra componentes en
//! [`BootsierRegions::Navbar`](bs::BootsierRegions::Navbar):
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::bs::BootsierRegions;
//!
//! InRegion::Global(&BootsierRegions::Navbar)
//!     .add(Html::with(|_| html! {
//!         li class="nav-item" {
//!             a class="nav-link" href="/logout" { "Cerrar sesión" }
//!         }
//!     }));
//! ```

pub mod bs;

pub mod class;

mod token;
pub use token::*;

#[doc(hidden)]
pub use bs::badge::BadgeBootsier;
#[doc(hidden)]
pub use bs::button::ButtonBootsier;
#[doc(hidden)]
pub use bs::container::ContainerBootsier;
#[doc(hidden)]
pub use bs::dropdown::DropdownBootsier;
#[doc(hidden)]
pub use bs::form::input::InputBootsier;
#[doc(hidden)]
pub use bs::form::select::SelectBootsier;
#[doc(hidden)]
pub use bs::form::textarea::TextareaBootsier;
#[doc(hidden)]
pub use bs::nav::NavBootsier;
#[doc(hidden)]
pub use bs::navbar::NavbarBootsier;
