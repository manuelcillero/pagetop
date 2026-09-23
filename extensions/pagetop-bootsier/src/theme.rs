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
//! Bootsier maqueta las dos plantillas de PageTop ([`CoreTemplates`]) con la misma cabecera,
//! contenido y pie: `Standard`, la plantilla por defecto de cualquier página; y `Admin`, que se
//! activa creando la página con [`Page::admin()`] en lugar de [`Page::new()`]. Ambas se maquetan
//! igual hoy -- la distinción entre página "normal" y de administración no vive en el maquetado,
//! sino en qué extensión registra contenido en cada región y para qué plantilla se autolimita (ver
//! más abajo `pagetop-admin`).
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
//! [`CoreTemplates`]: pagetop::prelude::CoreTemplates
//! [`Page::admin()`]: pagetop::response::Page::admin
//! [`Page::new()`]: pagetop::response::Page::new
//!
//! # Menú de administración
//!
//! `pagetop-admin` (si está presente en la aplicación) registra su propio menú de secciones y
//! páginas en [`CoreRegions::Aside`], autolimitado a
//! páginas con la plantilla `Admin` -- no requiere ningún registro por parte de Bootsier. Se
//! construye con [`Navbar`]/[`Nav`]/[`Dropdown`], los mismos
//! componentes disponibles para cualquier página de la aplicación.
//!
//! # Selector de modo de color
//!
//! [`bs::theme_toggle()`] crea un ítem de menú con el selector de modo de color (claro / oscuro /
//! automático), pensado para añadirse a cualquier [`Nav`] de la aplicación, no sólo a las
//! páginas de administración:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::*;
//!
//! let navbar = bs::Navbar::simple().with_item(bs::navbar::Item::nav(
//!     bs::Nav::new()
//!         .with_item(bs::nav::Item::link(Lc::n("Home"), "/"))
//!         .with_item(bs::theme_toggle()),
//! ));
//! ```

use pagetop::prelude::*;

pub mod bs;

pub mod class;

mod token;
pub use token::*;

#[doc(hidden)]
pub use bs::badge::BadgeBootsier;
#[doc(hidden)]
pub use bs::button::ButtonBootsier;
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

// Añade la clase de un punto de corte con un prefijo y un sufijo (opcional) a la cadena de clases,
// separada con un espacio de las que ya hubiera: `prefix-name-suffix`, sin `-suffix` si no hay
// sufijo, y sin `-name` si el punto de corte aplica siempre.
//
// `name` es el nombre del punto de corte según `Theme::breakpoint_entry()` en el tema activo de
// `cx`. Si el tema no le asocia un ancho mínimo (`Breakpoint::resolved()` devuelve `None`), el
// punto de corte aplica siempre y se omite `name`, igual que Bootstrap con `container` frente a
// `container-lg`, o `container-fluid` frente a `container-lg-fluid`.
pub(crate) fn push_breakpoint_class(
    cx: &Context,
    bp: Breakpoint,
    classes: &mut String,
    prefix: &str,
    suffix: &str,
) {
    if prefix.is_empty() {
        return;
    }
    if !classes.is_empty() {
        classes.push(' ');
    }
    classes.push_str(prefix);
    if let Some(entry) = bp.resolved(cx) {
        classes.push('-');
        classes.push_str(entry.name);
    }
    if !suffix.is_empty() {
        classes.push('-');
        classes.push_str(suffix);
    }
}
