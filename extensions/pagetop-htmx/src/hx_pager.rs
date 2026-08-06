//! Soporte HTMX al componente [`Pager`].
//!
//! [`Pager`] no conoce HTMX ni ninguna otra librería de interactividad. Cada enlace de página ya
//! funciona como una petición normal, y el elemento `<nav>` que los envuelve expone su propio
//! [`Props`], accesible con [`with_prop()`], que admite cualquier atributo HTML por nombre y valor
//! mediante [`PropsOp::set()`].
//!
//! Para una navegación sin recarga basta con añadir `hx::BOOST` (más `hx::TARGET` y `hx::SWAP`) al
//! propio `<nav>`. HTMX convierte automáticamente en peticiones AJAX los enlaces de todas las
//! páginas, reutilizando el `href` que cada uno ya tiene resuelto, sin que [`Pager`] tenga que
//! generar ningún atributo `hx-*` por sí misma.
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_htmx::prelude::*;
//!
//! let pager = Pager::new()
//!     .with_base_path("/admin/users")
//!     .with_extra_query("sort", "username")
//!     .with_current_page(2)
//!     .with_items_per_page(20)
//!     .with_total_items(97)
//!     .with_prop(PropsOp::set(hx::BOOST, "true"))
//!     .with_prop(PropsOp::set(hx::TARGET, "#user-table-wrapper"))
//!     .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
//!     .with_prop(PropsOp::set(hx::PUSH_URL, "true"));
//! ```
//!
//! [`Pager`]: pagetop::base::component::Pager
//! [`Props`]: pagetop::html::Props
//! [`PropsOp::set()`]: pagetop::html::PropsOp::set
//! [`with_prop()`]: pagetop::base::component::Pager::with_prop
