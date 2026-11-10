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
//! generar ningún atributo `hx-*` por sí misma. [`boost()`] los añade todos de una vez.
//!
//! [`Pager`]: pagetop::base::component::Pager
//! [`Props`]: pagetop::html::Props
//! [`PropsOp::set()`]: pagetop::html::PropsOp::set
//! [`with_prop()`]: pagetop::base::component::Pager::with_prop

use pagetop::prelude::*;

use crate::hx;

/// Hace que los enlaces de `pager` reemplacen sólo el elemento `target` (un selector CSS, p. ej.
/// `"#user-list"`), sin recargar la página.
///
/// Añade al `<nav>` del paginador `hx::BOOST`, `hx::TARGET`, `hx::SWAP` (reemplaza el elemento
/// entero y sube al principio) y `hx::PUSH_URL`, para que la dirección del navegador refleje la
/// página mostrada. Sin JavaScript, los enlaces siguen funcionando como peticiones normales.
///
/// Para que las peticiones no se pisen, una nueva petición contra `target` (desde los filtros, el
/// paginador o una cabecera) cancela la que estuviera en curso (`hx::SYNC`). Y mientras llega la
/// respuesta, `target` recibe la clase `htmx-request` (`hx::INDICATOR`), con la que la extensión
/// atenúa el listado ([`ListView`]).
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// let pager = hx_pager::boost(
///     Pager::new()
///         .with_base_path("/admin/users")
///         .with_extra_query("sort", "username")
///         .with_current_page(2)
///         .with_items_per_page(20)
///         .with_total_items(97),
///     "#user-list",
/// );
/// ```
pub fn boost(pager: Pager, target: impl Into<CowStr>) -> Pager {
    let target: CowStr = target.into();
    pager
        .with_prop(PropsOp::set(hx::BOOST, "true"))
        .with_prop(PropsOp::set(
            hx::SYNC,
            util::join!(target.as_ref(), ":replace"),
        ))
        .with_prop(PropsOp::set(hx::INDICATOR, target.clone()))
        .with_prop(PropsOp::set(hx::TARGET, target))
        .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
        .with_prop(PropsOp::set(hx::PUSH_URL, "true"))
}
