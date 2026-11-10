//! Soporte HTMX al componente [`ListFilter`].
//!
//! [`ListFilter`] es un formulario GET que funciona sin JavaScript: al aplicarlo, el navegador pide
//! de nuevo la página con los filtros aplicados. [`boost()`] hace que sólo se actualice el listado,
//! tanto al pulsar su botón como, con [`ListFilter::with_auto_submit()`], al cambiar un campo.

use pagetop::prelude::*;

use crate::hx;

/// Hace que, al aplicar `filter`, se actualice sólo el elemento `target` (un selector CSS, p. ej.
/// `"#user-list"`), sin recargar la página.
///
/// Añade al formulario `hx::BOOST` (envía sus campos a su propia ruta), `hx::TARGET`, `hx::SWAP`
/// (reemplaza el elemento entero y sube al principio) y `hx::PUSH_URL`, para que la dirección del
/// navegador refleje los filtros aplicados. No cambia cuándo se aplican: al pulsar el botón o, con
/// [`ListFilter::with_auto_submit()`], al cambiar un campo. Sin JavaScript, el filtro recarga la
/// página como cualquier formulario.
///
/// Para que las peticiones no se pisen, una nueva petición contra `target` (desde los filtros, el
/// paginador o una cabecera) cancela la que estuviera en curso (`hx::SYNC`). Y mientras llega la
/// respuesta, `target` recibe la clase `htmx-request` (`hx::INDICATOR`), con la que la extensión
/// atenúa el listado ([`ListView`]).
///
/// La ruta del filtro debe responder a las peticiones HTMX con el elemento `target` ya filtrado,
/// normalmente el [`ListView`] del listado.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// let filter = hx_filter::boost(
///     ListFilter::new()
///         .with_filter_action("/admin/users")
///         .with_field(form::input::Field::search().with_name("q")),
///     "#user-list",
/// );
/// ```
pub fn boost(filter: ListFilter, target: impl Into<CowStr>) -> ListFilter {
    let target: CowStr = target.into();
    filter
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
