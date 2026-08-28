//! Componentes para la barra lateral de la shell de Bootsier.
//!
//! # Componentes disponibles
//!
//! - [`Section`] - encabezado de grupo (`<li class="nav-header">`).
//! - [`Item`] - enlace de navegación con icono de Bootstrap Icons. Detecta
//!   automáticamente si la ruta activa coincide con la del *request* y añade la clase
//!   `active` al enlace.
//!
//! # Flujo de uso
//!
//! Los componentes se añaden a la región
//! [`BootsierRegions::Sidebar`](crate::theme::bs::BootsierRegions::Sidebar).
//! El registro global se hace una sola vez en el arranque; el registro por página
//! se hace al construir la página. La región sólo se renderiza en páginas creadas con
//! [`Page::admin()`](pagetop::response::Page::admin); en páginas con
//! [`Page::new()`](pagetop::response::Page::new) no tiene efecto.
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::bs::{BootsierRegions, sidebar};
//!
//! // Registro global: visible en todas las páginas de administración.
//! fn register_navigation() {
//!     InRegion::Global(&BootsierRegions::Sidebar)
//!         .add(sidebar::Section::titled(Lc::n("Administración")))
//!         .add(sidebar::Item::link(Lc::n("Usuarios"), "/users", "people"))
//!         .add(sidebar::Item::link(Lc::n("Roles"), "/roles", "shield-check"));
//! }
//!
//! // Uso en un handler: la página de administración muestra el sidebar registrado.
//! async fn users(request: HttpRequest) -> Result<Markup, ErrorPage> {
//!     Page::admin(request)
//!         .with_child(Html::with(|_| html! { h3 { "Usuarios" } }))
//!         .render().await
//! }
//! ```

mod item;
pub use item::Item;

mod section;
pub use section::Section;
