//! Soporte HTMX al componente [`Table`].
//!
//! [`Table`] no conoce HTMX ni ninguna otra librería de interactividad. En su lugar, cada pieza que
//! necesita comportamiento interactivo ([`table::Row`], [`table::Cell`], [`table::SortLink`])
//! expone su propio [`Props`] que admite cualquier atributo HTML por nombre y valor mediante
//! [`PropsOp::set()`], el mismo mecanismo que usa el resto de PageTop para adoptar HTMX sin
//! depender de él. Basta con encadenar `.with_prop(PropsOp::set(hx::GET, ...))` sobre el elemento
//! que corresponda.
//!
//! Las cabeceras ordenables ([`table::Column::with_sort`]) ya llevan resuelta la parte que no
//! depende de HTMX: el enlace (`<a href>`), la semántica accesible (`aria-sort`) y el indicador
//! visual de dirección (clases CSS). Sólo falta (si se quiere navegación sin recarga) añadir los
//! atributos `hx-*` con [`table::SortLink::with_prop()`], o usar directamente [`sort_link()`] para
//! no repetirlos en cada columna.

use pagetop::prelude::*;

use crate::hx;

// **< sort_link() >********************************************************************************

/// Construye un [`SortLink`] para actualizar el orden de la tabla sin recargar la página.
///
/// [`Table`] y `SortLink` no requieren HTMX. Cada extensión que quiera aplicar una navegación sin
/// recarga debe añadir sus propios atributos `hx-*` usando [`SortLink::with_prop()`]. Como esos
/// cuatro atributos son siempre los mismos para cualquier cabecera ordenable (`hx-get` igual al
/// `href`, `hx-swap="outerHTML scroll:top"` y `hx-push-url="true"`, y sólo `hx-target` cambia según
/// la tabla), [`sort_link()`] evita reescribirlos en cada columna de cada listado.
///
/// El enlace resultante funciona igual con o sin HTMX: `href` es siempre la URL real del nuevo
/// estado de orden, así que navega correctamente aunque HTMX no esté disponible en el cliente.
///
/// # Argumentos
///
/// - `href`: URL completa hacia el nuevo estado de orden, reflejando ya el campo y la dirección
///   que resultarán de pulsar esta cabecera. Acepta cualquier tipo convertible a [`RoutePath`],
///   normalmente el resultado de [`Context::route()`], para que el enlace preserve el parámetro
///   `lang` cuando corresponda.
/// - `target`: selector CSS del elemento que HTMX debe reemplazar (`hx-target`), típicamente el
///   contenedor que envuelve la tabla completa.
/// - `dir`: dirección de orden vigente de esta columna, o `None` si la tabla está ordenada
///   actualmente por otra columna. Se traslada tal cual a [`SortLink::with_dir()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_htmx::prelude::*;
///
/// # fn build_column(cx: &Context) -> table::Column {
/// let current_field = "username";                 // Estado vigente de la tabla.
/// let current_dir = html::SortDir::Asc;           // Ordenada por "username" en ascendente.
///
/// let field = "username";                         // Cabecera de la propia columna "username".
/// let is_active = field == current_field;         // En el ejemplo, coincide con el campo vigente.
/// let active = is_active.then_some(current_dir);  // `Some` sólo si esta columna ordena ahora.
/// let next_dir = html::SortDir::next_for(active); // El siguiente clic alterna la dirección.
///
/// // `cx` es el `Context` de la petición en curso.
/// let href = cx
///     .route("/admin/users")
///     .with_param("sort", field)
///     .with_param("dir", next_dir);
///
/// table::Column::new(L10n::n("User"))
///     .with_sort(hx_table::sort_link(href, "#user-table-wrapper", active))
/// # }
/// ```
///
/// [`SortLink`]: pagetop::base::component::table::SortLink
/// [`SortLink::with_prop()`]: pagetop::base::component::table::SortLink::with_prop
/// [`SortLink::with_dir()`]: pagetop::base::component::table::SortLink::with_dir
/// [`Context::route()`]: pagetop::core::component::Context::route
pub fn sort_link(
    href: impl Into<RoutePath>,
    target: impl AsRef<str>,
    dir: impl Into<Option<SortDir>>,
) -> table::SortLink {
    // Se materializa como `String` propio porque el mismo valor sirve para dos llamadas: como
    // `RoutePath` en `SortLink::new()` (vía `href.as_str()`) y como `CowStr` en `PropsOp::set()`.
    let href = href.into().to_string();
    let target = target.as_ref().to_owned();
    table::SortLink::new(href.as_ref())
        .with_dir(dir)
        .with_prop(PropsOp::set(hx::GET, href))
        .with_prop(PropsOp::set(hx::TARGET, target))
        .with_prop(PropsOp::set(hx::SWAP, hx::swap::OUTER_HTML_SCROLL_TOP))
        .with_prop(PropsOp::set(hx::PUSH_URL, "true"))
}
